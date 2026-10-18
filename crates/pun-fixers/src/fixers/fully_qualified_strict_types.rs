//! Port of `PhpCsFixer\Fixer\Import\FullyQualifiedStrictTypesFixer`.
//!
//! Every `$importKind` upstream passes is `'class'` (uses are collected for class imports only), so the
//! per-kind dimension of `$uses`, `$discoveredSymbols`, `$symbolsForImport` and the uses caches is dropped.
use crate::analyzers::analysis::{NamespaceUseType, TypeAnalysis};
use crate::analyzers::attribute_analyzer::AttributeAnalyzer;
use crate::analyzers::docblock::{
    TypeExpression, type_expression::REGEX_IDENTIFIER, type_expression::REGEX_TYPES,
};
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use crate::analyzers::import_processor::ImportProcessor;
use crate::analyzers::namespace_uses_analyzer::NamespaceUsesAnalyzer;
use pun_tokens::NamespacesAnalyzer;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use indexmap::IndexMap;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::php_compat::{ltrim_chars, strnatcmp, strtolower, substr_count};
use pun_tokens::{Kind::*, Token, Tokens};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use std::sync::LazyLock;

const PHPDOC_TAGS: &[&str] = &[
    "param",
    "phpstan-param",
    "phpstan-property",
    "phpstan-property-read",
    "phpstan-property-write",
    "phpstan-return",
    "phpstan-var",
    "property",
    "property-read",
    "property-write",
    "psalm-param",
    "psalm-property",
    "psalm-property-read",
    "psalm-property-write",
    "psalm-return",
    "psalm-var",
    "return",
    "see",
    "throws",
    "var",
];

fn regex_class() -> String {
    format!(r"(?:\\?+{REGEX_IDENTIFIER}(\\{REGEX_IDENTIFIER})*+)")
}

/// A PHP array with string keys: insertion-ordered, assigning an existing key keeps its position.
type OMap = IndexMap<Vec<u8>, Vec<u8>>;

#[derive(Default)]
struct FullyQualifiedStrictTypesFixer {
    leading_backslash_in_global_namespace: bool,
    import_symbols: bool,
    phpdoc_tags: Vec<String>,
    whitespaces_config: WhitespacesFixerConfig,
}

/// Per-run mutable state (upstream's private properties).
#[derive(Default)]
struct State {
    /// `null` outside the discovery phase.
    discovered_symbols: RefCell<Option<Vec<Vec<u8>>>>,
    /// short name => FQCN
    symbols_for_import: OMap,
    reserved_identifiers_by_level: BTreeMap<i64, HashSet<Vec<u8>>>,
    /// `cacheUsesLast` and the maps built from it (upstream rebuilds only when `$uses` differs).
    uses_cache: RefCell<Option<(OMap, std::rc::Rc<UsesCache>)>>,
}

/// `refreshUsesCache()` output.
struct UsesCache {
    use_name_by_short_name_lower: HashMap<Vec<u8>, Vec<u8>>,
    use_short_name_by_name: HashMap<Vec<u8>, Vec<u8>>,
    use_short_name_by_normalized_name: HashMap<Vec<u8>, Vec<u8>>,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(FullyQualifiedStrictTypesFixer::default())
}

impl Fixer for FullyQualifiedStrictTypesFixer {
    fn name(&self) -> &'static str {
        "fully_qualified_strict_types"
    }
    fn priority(&self) -> i32 {
        7
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[
            CT_USE_TRAIT,
            T_ATTRIBUTE,
            T_CATCH,
            T_DOUBLE_COLON,
            T_DOC_COMMENT,
            T_EXTENDS,
            T_FUNCTION,
            T_IMPLEMENTS,
            T_INSTANCEOF,
            T_NEW,
            T_VARIABLE,
        ])
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(
            self.name(),
            options,
            &[
                "leading_backslash_in_global_namespace",
                "import_symbols",
                "phpdoc_tags",
            ],
        )?;
        self.leading_backslash_in_global_namespace =
            r.bool("leading_backslash_in_global_namespace", false)?;
        self.import_symbols = r.bool("import_symbols", false)?;
        self.phpdoc_tags = r.strings("phpdoc_tags", PHPDOC_TAGS, &[])?;
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        let mut st = State::default();
        let regex_comment_attribute = format!(r"/#\[\s*({})/", regex_class());
        let regex_reserved = format!(
            r"/\*\h*@(?:psalm-|phpstan-)?(?:template(?:-covariant|-contravariant)?|(?:import-)?type)\h+({REGEX_IDENTIFIER})(?!\S)/i"
        );

        let namespace_count = NamespacesAnalyzer.get_declarations(tokens).len();
        for namespace_index in 0..namespace_count {
            // `$tokens->getNamespaceDeclarations()` is recomputed after every change.
            let namespace = NamespacesAnalyzer
                .get_declarations(tokens)
                .swap_remove(namespace_index);
            let namespace_name = namespace.full_name.clone();

            let mut uses = OMap::default();
            let mut last_use = None;
            for u in NamespaceUsesAnalyzer.get_declarations_in_namespace(tokens, &namespace, true) {
                if !u.is_class() {
                    continue;
                }
                let full_name = ltrim_chars(&u.full_name, b"\\").to_vec();
                uses.insert(full_name, u.short_name.clone());
                last_use = Some(u);
            }

            let mut index_diff: isize = 0;
            let phases: &[bool] = if self.import_symbols {
                &[true, false]
            } else {
                &[false]
            };
            for &discover_symbols_phase in phases {
                *st.discovered_symbols.borrow_mut() = discover_symbols_phase.then(Vec::new);
                let mut opened_curly_brackets: i64 = 0;
                st.reserved_identifiers_by_level.clear();

                let mut index = namespace.scope_start_index;
                while (index as isize) < namespace.scope_end_index as isize + index_diff {
                    let orig_size = tokens.count();
                    // Only comment branches read the content after edits; other tokens need just their kind.
                    let t = &tokens[index];
                    let (open, close) = (t.equals("{", true), t.equals("}", true));
                    let token = if t.is_comment() {
                        t.clone()
                    } else {
                        Token {
                            id: t.id,
                            content: Vec::new(),
                        }
                    };
                    if open {
                        opened_curly_brackets += 1;
                    } else if close {
                        st.reserved_identifiers_by_level
                            .remove(&opened_curly_brackets);
                        opened_curly_brackets -= 1;
                        assert!(opened_curly_brackets >= 0);
                    } else if token.is_kind(T_VARIABLE) {
                        if tokens
                            .get_prev_meaningful_token(index)
                            .is_some_and(|p| tokens[p].is_kind(T_STRING))
                        {
                            self.fix_prev_name(&st, tokens, index, &uses, &namespace_name);
                        }
                    } else if token.is_kind(T_DOUBLE_COLON) {
                        self.fix_prev_name(&st, tokens, index, &uses, &namespace_name);
                    } else if token.is_kind(T_FUNCTION) {
                        self.fix_function(
                            &st,
                            &functions_analyzer,
                            tokens,
                            index,
                            &uses,
                            &namespace_name,
                        );
                    } else if token.is_kind(T_ATTRIBUTE) {
                        self.fix_attribute(&st, tokens, index, &uses, &namespace_name);
                    } else if token.is_kind(T_CATCH) {
                        self.fix_catch(&st, tokens, index, &uses, &namespace_name);
                    } else if discover_symbols_phase && token.is_classy() {
                        self.fix_next_name(&st, tokens, index, &uses, &namespace_name);
                    } else if token.is_given_kind(&[T_EXTENDS, T_IMPLEMENTS]) {
                        self.fix_extends_implements(&st, tokens, index, &uses, &namespace_name);
                    } else if token.is_given_kind(&[
                        T_INSTANCEOF,
                        T_NEW,
                        CT_USE_TRAIT,
                        CT_TYPE_COLON,
                    ]) {
                        self.fix_next_name(&st, tokens, index, &uses, &namespace_name);
                    } else if let Some(m) = (discover_symbols_phase && token.is_kind(T_COMMENT))
                        .then(|| Preg::match_groups(&regex_comment_attribute, &token.content))
                        .flatten()
                    {
                        // @TODO: drop when PHP 8.0+ is required
                        self.determine_short_type(&st, m.str(1), &uses, &namespace_name);
                    } else if token.is_kind(T_DOC_COMMENT) {
                        for m in Preg::match_all(&regex_reserved, &token.content) {
                            st.reserved_identifiers_by_level
                                .entry(opened_curly_brackets + 1)
                                .or_default()
                                .insert(m.str(1).to_vec());
                        }
                        self.fix_php_doc(&st, tokens, index, &uses, &namespace_name);
                    }
                    index_diff += tokens.count() as isize - orig_size as isize;
                    index += 1;
                }

                st.reserved_identifiers_by_level.clear();
                if discover_symbols_phase {
                    self.setup_uses_from_discovered_symbols(&mut st, &mut uses, &namespace_name);
                }
            }

            if !st.symbols_for_import.is_empty() {
                let at_index = if let Some(last_use) = &last_use {
                    last_use.end_index + 1
                } else if namespace.end_index != 0 {
                    namespace.end_index + 1
                } else {
                    match tokens.get_next_meaningful_token(namespace.scope_start_index) {
                        Some(first) if tokens[first].is_kind(T_DECLARE) => {
                            tokens
                                .get_next_token_of_kind(first, &[";".into()], true)
                                .unwrap()
                                + 1
                        }
                        _ => namespace.scope_start_index + 1,
                    }
                };
                let names = st
                    .symbols_for_import
                    .values()
                    .cloned()
                    .collect();
                ImportProcessor::new(self.whitespaces_config.clone()).insert_imports(
                    tokens,
                    &[(NamespaceUseType::Class, names)],
                    at_index,
                );
                st.symbols_for_import = OMap::default();
            }
        }
    }
}

fn refresh_uses_cache(st: &State, uses: &OMap) -> std::rc::Rc<UsesCache> {
    if let Some((last, c)) = &*st.uses_cache.borrow()
        && last.iter().eq(uses)
    {
        return c.clone();
    }
    let c = std::rc::Rc::new(build_uses_cache(uses));
    *st.uses_cache.borrow_mut() = Some((uses.clone(), c.clone()));
    c
}

fn build_uses_cache(uses: &OMap) -> UsesCache {
    let mut c = UsesCache {
        use_name_by_short_name_lower: HashMap::new(),
        use_short_name_by_name: HashMap::new(),
        use_short_name_by_normalized_name: HashMap::new(),
    };
    for (use_long_name, use_short_name) in uses {
        c.use_name_by_short_name_lower
            .insert(strtolower(use_short_name), use_long_name.clone());
        c.use_short_name_by_name
            .insert(use_long_name.clone(), use_short_name.clone());
        c.use_short_name_by_normalized_name
            .insert(normalize_fqcn(use_long_name), use_short_name.clone());
    }
    c
}

/// `explode('\\', $s, 2)[0]`
fn first_part(s: &[u8]) -> &[u8] {
    s.split(|&c| c == b'\\').next().unwrap()
}

/// `strrpos($s, '\\')`
fn last_backslash(s: &[u8]) -> Option<usize> {
    s.iter().rposition(|&c| c == b'\\')
}

fn normalize_fqcn(input: &[u8]) -> Vec<u8> {
    match last_backslash(input) {
        None => strtolower(input),
        Some(p) => [&input[..=p], &strtolower(&input[p + 1..])[..]].concat(),
    }
}

fn namespaced_string_to_tokens(mut input: &[u8]) -> Vec<Token> {
    let mut tokens = Vec::new();
    if let Some(rest) = input.strip_prefix(b"\\") {
        tokens.push(Token::new(T_NS_SEPARATOR, "\\"));
        input = rest;
    }
    let parts: Vec<&[u8]> = input.split(|&c| c == b'\\').collect();
    for (index, part) in parts.iter().enumerate() {
        tokens.push(Token::new(T_STRING, *part));
        if index != parts.len() - 1 {
            tokens.push(Token::new(T_NS_SEPARATOR, "\\"));
        }
    }
    tokens
}

impl FullyQualifiedStrictTypesFixer {
    fn is_reserved_identifier(&self, st: &State, symbol: &[u8]) -> bool {
        if symbol.contains(&b'\\') {
            // optimization only
            return false;
        }
        if TypeAnalysis::new(symbol, None, None).is_reserved_type() {
            return true;
        }
        st.reserved_identifiers_by_level
            .values()
            .any(|r| r.contains(symbol))
    }

    fn resolve_symbol(
        &self,
        st: &State,
        symbol: &[u8],
        uses: &OMap,
        namespace_name: &[u8],
    ) -> Vec<u8> {
        if let Some(rest) = symbol.strip_prefix(b"\\") {
            return rest.to_vec();
        }
        if self.is_reserved_identifier(st, symbol) {
            return symbol.to_vec();
        }
        let cache = refresh_uses_cache(st, uses);
        let mut symbol_arr = symbol.splitn(2, |&c| c == b'\\');
        let short_start_name_lower = strtolower(symbol_arr.next().unwrap());
        if let Some(long) = cache
            .use_name_by_short_name_lower
            .get(&short_start_name_lower)
        {
            return match symbol_arr.next() {
                Some(rest) => [&long[..], b"\\", rest].concat(),
                None => long.clone(),
            };
        }
        if namespace_name.is_empty() {
            symbol.to_vec()
        } else {
            [namespace_name, b"\\", symbol].concat()
        }
    }

    fn shorten_symbol(
        &self,
        st: &State,
        fqcn: &[u8],
        uses: &OMap,
        namespace_name: &[u8],
    ) -> Vec<u8> {
        if self.is_reserved_identifier(st, fqcn) {
            return fqcn.to_vec();
        }
        let cache = refresh_uses_cache(st, uses);
        let mut res: Option<Vec<u8>> = None;

        let mut i_min = 0;
        let ns_prefix = [namespace_name, b"\\"].concat();
        if let Some(tmp_res) = fqcn.strip_prefix(&ns_prefix[..])
            && !cache
                .use_name_by_short_name_lower
                .contains_key(&strtolower(first_part(tmp_res)))
            && !self.is_reserved_identifier(st, tmp_res)
        {
            res = Some(tmp_res.to_vec());
            i_min = substr_count(namespace_name, b"\\") + 1;
        }

        let mut tmp = fqcn;
        let mut i = substr_count(fqcn, b"\\") as isize;
        while i >= i_min as isize {
            if let Some(short) = cache.use_short_name_by_name.get(tmp) {
                let tmp_res = [&short[..], &fqcn[tmp.len()..]].concat();
                if !self.is_reserved_identifier(st, &tmp_res) {
                    res = Some(tmp_res);
                    break;
                }
            }
            if i > 0 {
                tmp = &tmp[..last_backslash(tmp).unwrap()];
            }
            i -= 1;
        }

        if res.is_none()
            && let Some(tmp_res) = cache
                .use_short_name_by_normalized_name
                .get(&normalize_fqcn(fqcn))
            && !self.is_reserved_identifier(st, tmp_res)
        {
            res = Some(tmp_res.clone());
        }

        res.unwrap_or_else(|| {
            if !namespace_name.is_empty()
                || self.leading_backslash_in_global_namespace
                || cache
                    .use_name_by_short_name_lower
                    .contains_key(&strtolower(first_part(fqcn)))
            {
                [b"\\", fqcn].concat()
            } else {
                fqcn.to_vec()
            }
        })
    }

    fn setup_uses_from_discovered_symbols(
        &self,
        st: &mut State,
        uses: &mut OMap,
        namespace_name: &[u8],
    ) {
        let mut discovered_symbols = st.discovered_symbols.borrow_mut().take().unwrap_or_default();
        // upstream iterates `$this->discoveredSymbols` per kind; no kind = nothing discovered
        if discovered_symbols.is_empty() {
            return;
        }
        let mut discovered_fqcn_by_short_name_lower = OMap::default();

        if namespace_name.is_empty() {
            for symbol in &discovered_symbols {
                if !symbol.starts_with(b"\\") {
                    let short_start_name = first_part(ltrim_chars(symbol, b"\\"));
                    let resolved = self.resolve_symbol(st, short_start_name, uses, namespace_name);
                    discovered_fqcn_by_short_name_lower.insert(strtolower(short_start_name), resolved);
                }
            }
        }

        for (use_long_name, use_short_name) in uses.iter() {
            discovered_fqcn_by_short_name_lower
                .insert(strtolower(use_short_name), use_long_name.clone());
        }

        let use_by_short_name_lower: HashSet<Vec<u8>> =
            uses.iter().map(|(_, short)| strtolower(short)).collect();

        discovered_symbols.sort_by(|a, b| {
            a.starts_with(b"\\")
                .cmp(&b.starts_with(b"\\"))
                .then(substr_count(a, b"\\").cmp(&substr_count(b, b"\\")))
        });
        for symbol in &discovered_symbols {
            let mut symbol = &symbol[..];
            loop {
                let short_end_name_lower = strtolower(match last_backslash(symbol) {
                    Some(p) => &symbol[p + 1..],
                    None => symbol,
                });
                if discovered_fqcn_by_short_name_lower
                    .get(&short_end_name_lower)
                    .is_none()
                {
                    let short_start_name_lower = strtolower(first_part(ltrim_chars(symbol, b"\\")));
                    if symbol.starts_with(b"\\")
                        || (namespace_name.is_empty()
                            && !use_by_short_name_lower.contains(&short_start_name_lower))
                        || !symbol.contains(&b'\\')
                    {
                        let resolved = self.resolve_symbol(st, symbol, uses, namespace_name);
                        discovered_fqcn_by_short_name_lower.insert(short_end_name_lower, resolved);
                        break;
                    }
                }
                // else short name collision - keep unimported

                if symbol.starts_with(b"\\")
                    || namespace_name.is_empty()
                    || !symbol.contains(&b'\\')
                {
                    break;
                }
                symbol = &symbol[..last_backslash(symbol).unwrap()];
            }
        }

        for (use_long_name, use_short_name) in uses.iter() {
            let key = strtolower(use_short_name);
            let discovered_long_name = discovered_fqcn_by_short_name_lower
                .get(&key)
                .cloned()
                .unwrap_or_default();
            if strtolower(&discovered_long_name) == strtolower(use_long_name) {
                discovered_fqcn_by_short_name_lower.shift_remove(&key);
            }
        }

        for (_, fqcn) in &discovered_fqcn_by_short_name_lower {
            let shortened = self.shorten_symbol(st, fqcn, &OMap::default(), namespace_name);
            let shortened_name = ltrim_chars(&shortened, b"\\");
            if shortened_name.contains(&b'\\') {
                // prevent importing non-namespaced names in global namespace
                // (the shortened name is a suffix of the FQCN, so the FQCN has a backslash too)
                let short_end_name = fqcn[last_backslash(fqcn).unwrap() + 1..].to_vec();
                uses.insert(fqcn.clone(), short_end_name.clone());
                st.symbols_for_import.insert(short_end_name, fqcn.clone());
            }
        }

        // ksort(SORT_NATURAL), stable like PHP's sort
        st.symbols_for_import
            .sort_by(|a, _, b, _| strnatcmp(a, b).cmp(&0));
    }

    fn fix_function(
        &self,
        st: &State,
        functions_analyzer: &FunctionsAnalyzer,
        tokens: &mut Tokens,
        index: usize,
        uses: &OMap,
        ns: &[u8],
    ) {
        let names: Vec<Vec<u8>> = functions_analyzer
            .get_function_arguments(tokens, index)
            .into_iter()
            .map(|(n, _)| n)
            .collect();
        for name in names {
            let argument = functions_analyzer
                .get_function_arguments(tokens, index)
                .into_iter()
                .find(|(n, _)| *n == name)
                .unwrap()
                .1;
            if let Some(type_analysis) = &argument.type_analysis {
                self.replace_by_short_type(st, tokens, type_analysis, uses, ns);
            }
        }
        if let Some(return_type_analysis) =
            functions_analyzer.get_function_return_type(tokens, index)
        {
            self.replace_by_short_type(st, tokens, &return_type_analysis, uses, ns);
        }
    }

    fn fix_php_doc(&self, st: &State, tokens: &mut Tokens, index: usize, uses: &OMap, ns: &[u8]) {
        let allowed_tags = &self.phpdoc_tags;
        if allowed_tags.is_empty() {
            return;
        }
        let php_doc_content = tokens[index].content.clone();
        static DOC_TAG: LazyLock<String> =
            LazyLock::new(|| format!(r"/([*{{]\h*@)(\S+)(\h+)({REGEX_TYPES})(?!(?!\}})\S)/"));
        static SEE_TAG: LazyLock<String> = LazyLock::new(|| {
            format!(
                r"/([*{{]\h*)@see(\h+)({})(::(?:\$\w+|\w+\(\)))(?!(?!\}})\S)/",
                regex_class()
            )
        });
        let mut php_doc_content_new = Preg::replace_callback(
            &*DOC_TAG,
            |m| {
                if !allowed_tags.iter().any(|t| t.as_bytes() == m.str(2)) {
                    return m.str(0).to_vec();
                }
                [
                    m.str(1),
                    m.str(2),
                    m.str(3),
                    &self.fix_php_doc_type(st, m.str(4), uses, ns),
                ]
                .concat()
            },
            &php_doc_content,
        );
        if allowed_tags.iter().any(|t| t == "see") {
            php_doc_content_new = Preg::replace_callback(
                &*SEE_TAG,
                |m| {
                    [
                        m.str(1),
                        b"@see",
                        m.str(2),
                        &self.fix_php_doc_type(st, m.str(3), uses, ns),
                        m.str(5),
                    ]
                    .concat()
                },
                &php_doc_content_new,
            );
        }
        if php_doc_content_new != php_doc_content {
            tokens.set(index, Token::new(T_DOC_COMMENT, php_doc_content_new));
        }
    }

    fn fix_php_doc_type(&self, st: &State, type_: &[u8], uses: &OMap, ns: &[u8]) -> Vec<u8> {
        static CLASS: LazyLock<String> = LazyLock::new(|| format!("/^{}$/", regex_class()));
        let regex: &str = &CLASS;
        let type_expression = TypeExpression::new(type_).map_types(&mut |type_| {
            let current_type_value = type_.to_string().to_vec();
            if type_.is_composite_type()
                || !Preg::r#match(regex, &current_type_value)
                || current_type_value == b"min"
                || current_type_value == b"max"
            {
                return type_;
            }
            let Some(short_tokens) = self.determine_short_type(st, &current_type_value, uses, ns)
            else {
                return type_;
            };
            let new_type_value: Vec<u8> = short_tokens
                .iter()
                .flat_map(|t| t.content.iter().copied())
                .collect();
            // upstream keeps `$type` when the value is unchanged, but determineShortType returns null then
            TypeExpression::new(&new_type_value)
        });
        type_expression.to_string().to_vec()
    }

    fn fix_extends_implements(
        &self,
        st: &State,
        tokens: &mut Tokens,
        index: usize,
        uses: &OMap,
        ns: &[u8],
    ) {
        // We handle `extends` and `implements` with similar logic, but we need to exit the loop under different conditions.
        let is_extends = tokens[index].is_kind(T_EXTENDS);
        let mut index = tokens.get_next_meaningful_token(index).unwrap();
        let mut type_start_index = None;
        let mut type_end_index = 0;
        loop {
            if tokens[index].equals_any(&[",".into(), "{".into(), T_IMPLEMENTS.into()], true) {
                if let Some(start) = type_start_index {
                    index = index.wrapping_add_signed(self.shorten_class_if_possible(
                        st,
                        tokens,
                        start,
                        type_end_index,
                        uses,
                        ns,
                    ));
                }
                type_start_index = None;
                let stop: &[pun_tokens::Proto] = if is_extends {
                    &[T_IMPLEMENTS.into(), "{".into()]
                } else {
                    &["{".into()]
                };
                if tokens[index].equals_any(stop, true) {
                    break;
                }
            } else {
                type_start_index.get_or_insert(index);
                type_end_index = index;
            }
            index = tokens.get_next_meaningful_token(index).unwrap();
        }
    }

    fn fix_catch(&self, st: &State, tokens: &mut Tokens, index: usize, uses: &OMap, ns: &[u8]) {
        let index = tokens.get_next_meaningful_token(index).unwrap();
        let mut index = tokens.get_next_meaningful_token(index).unwrap(); // first part of first exception class to be caught
        let mut type_start_index = None;
        let mut type_end_index = 0;
        loop {
            if tokens[index].equals_any(
                &[")".into(), T_VARIABLE.into(), CT_TYPE_ALTERNATION.into()],
                true,
            ) {
                let Some(start) = type_start_index else { break };
                index = index.wrapping_add_signed(self.shorten_class_if_possible(
                    st,
                    tokens,
                    start,
                    type_end_index,
                    uses,
                    ns,
                ));
                type_start_index = None;
                if tokens[index].equals(")", true) {
                    break;
                }
            } else {
                type_start_index.get_or_insert(index);
                type_end_index = index;
            }
            index = tokens.get_next_meaningful_token(index).unwrap();
        }
    }

    fn fix_attribute(&self, st: &State, tokens: &mut Tokens, index: usize, uses: &OMap, ns: &[u8]) {
        let attribute_analysis = AttributeAnalyzer::collect_one(tokens, index);
        for attribute in &attribute_analysis.attributes {
            let mut index = attribute.start;
            while tokens[index].is_given_kind(&[T_STRING, T_NS_SEPARATOR]) {
                index = tokens.get_prev_meaningful_token(index).unwrap();
            }
            self.fix_next_name(st, tokens, index, uses, ns);
        }
    }

    fn fix_prev_name(
        &self,
        st: &State,
        tokens: &mut Tokens,
        mut index: usize,
        uses: &OMap,
        ns: &[u8],
    ) {
        let mut type_start_index = 0;
        let mut type_end_index = None;
        loop {
            index = tokens.get_prev_meaningful_token(index).unwrap();
            if tokens[index].is_object_operator() {
                break;
            }
            if tokens[index].is_given_kind(&[T_STRING, T_NS_SEPARATOR]) {
                type_start_index = index;
                type_end_index.get_or_insert(index);
            } else {
                if let Some(end) = type_end_index {
                    self.shorten_class_if_possible(st, tokens, type_start_index, end, uses, ns);
                }
                break;
            }
        }
    }

    fn fix_next_name(
        &self,
        st: &State,
        tokens: &mut Tokens,
        mut index: usize,
        uses: &OMap,
        ns: &[u8],
    ) {
        let mut type_start_index = None;
        let mut type_end_index = 0;
        loop {
            index = tokens.get_next_meaningful_token(index).unwrap();
            if tokens[index].is_given_kind(&[T_STRING, T_NS_SEPARATOR]) {
                type_start_index.get_or_insert(index);
                type_end_index = index;
            } else {
                if let Some(start) = type_start_index {
                    self.shorten_class_if_possible(st, tokens, start, type_end_index, uses, ns);
                }
                break;
            }
        }
    }

    fn shorten_class_if_possible(
        &self,
        st: &State,
        tokens: &mut Tokens,
        start: usize,
        end: usize,
        uses: &OMap,
        ns: &[u8],
    ) -> isize {
        let content = tokens.generate_partial_code(start, end);
        let Some(new_tokens) = self.determine_short_type(st, &content, uses, ns) else {
            return 0;
        };
        let n = new_tokens.len() as isize;
        tokens.override_range(start, end, new_tokens);
        n - (end - start) as isize - 1
    }

    fn replace_by_short_type(
        &self,
        st: &State,
        tokens: &mut Tokens,
        type_: &TypeAnalysis,
        uses: &OMap,
        ns: &[u8],
    ) {
        let mut index = type_.get_start_index();
        if tokens[index].is_kind(CT_NULLABLE_TYPE) {
            index = tokens.get_next_meaningful_token(index).unwrap();
        }
        let mut end_index = type_.get_end_index();

        // inlined `getTypes()` generator; the consumer's work happens at the `yield`
        let mut skip_next_yield = false;
        let (mut type_start_index, mut type_end_index): (Option<usize>, usize) = (None, 0);
        loop {
            if tokens[index].is_kind(CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_OPEN) {
                index = tokens.get_next_meaningful_token(index).unwrap();
                type_start_index = None;
                continue;
            }
            if tokens[index].is_given_kind(&[
                CT_TYPE_ALTERNATION,
                CT_TYPE_INTERSECTION,
                CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_CLOSE,
            ]) || index > end_index
            {
                match type_start_index.filter(|_| !skip_next_yield) {
                    Some(start) => {
                        let orig_count = tokens.count();
                        let content = tokens.generate_partial_code(start, type_end_index);
                        if let Some(new_tokens) = self.determine_short_type(st, &content, uses, ns)
                        {
                            tokens.override_range(start, type_end_index, new_tokens);
                        }
                        end_index += tokens.count() - orig_count;
                        // type tokens were possibly updated, restart type match
                        skip_next_yield = true;
                        index = start;
                        type_end_index = start;
                    }
                    None => {
                        skip_next_yield = false;
                        index = tokens.get_next_meaningful_token(index).unwrap();
                        type_start_index = None;
                    }
                }
                if index > end_index {
                    break;
                }
                continue;
            }
            type_start_index.get_or_insert(index);
            type_end_index = index;
            index = tokens.get_next_meaningful_token(index).unwrap();
        }
    }

    fn determine_short_type(
        &self,
        st: &State,
        type_name: &[u8],
        uses: &OMap,
        ns: &[u8],
    ) -> Option<Vec<Token>> {
        if st.discovered_symbols.borrow().is_some() {
            if !self.is_reserved_identifier(st, type_name) {
                st.discovered_symbols
                    .borrow_mut()
                    .as_mut()
                    .unwrap()
                    .push(type_name.to_vec());
            }
            return None;
        }
        let fqcn = self.resolve_symbol(st, type_name, uses, ns);
        let shortened_type = self.shorten_symbol(st, &fqcn, uses, ns);
        if shortened_type == type_name {
            return None;
        }
        Some(namespaced_string_to_tokens(&shortened_type))
    }
}
