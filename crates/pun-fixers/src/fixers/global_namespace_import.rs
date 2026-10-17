//! Port of `PhpCsFixer\Fixer\Import\GlobalNamespaceImportFixer`.
use crate::analyzers::analysis::NamespaceUseAnalysis;
use crate::analyzers::classy_analyzer::ClassyAnalyzer;
use crate::analyzers::constant_invocation::is_constant_invocation;
use crate::analyzers::docblock::{DocBlock, TAGS_WITH_TYPES};
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use crate::analyzers::import_processor::{ImportProcessor, ImportType};
use crate::analyzers::namespace_uses_analyzer::NamespaceUsesAnalyzer;
use crate::analyzers::namespaces_analyzer::NamespacesAnalyzer;
use crate::analyzers::preg::{PREG_OFFSET_CAPTURE, Preg};
use anyhow::Result;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{
    Kind::*,
    Token, Tokens,
    php_compat::{ltrim_chars, strtolower},
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

struct GlobalNamespaceImportFixer {
    import_constants: Option<bool>,
    import_functions: Option<bool>,
    import_classes: Option<bool>,
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(GlobalNamespaceImportFixer {
        import_constants: None,
        import_functions: None,
        import_classes: Some(true),
        whitespaces_config: Default::default(),
    })
}

/// `array<string, string|true>`: `None` = `true` (not aliased), `Some(alias)`.
type Global = BTreeMap<Vec<u8>, Option<Vec<u8>>>;
type Other = BTreeSet<Vec<u8>>;

/// Ordered `array<string, string>` with PHP assignment semantics (existing key keeps its position).
#[derive(Default)]
struct Imports(Vec<(Vec<u8>, Vec<u8>)>);

impl Imports {
    fn set(&mut self, k: Vec<u8>, v: Vec<u8>) {
        match self.0.iter_mut().find(|(kk, _)| *kk == k) {
            Some(e) => e.1 = v,
            None => self.0.push((k, v)),
        }
    }
    fn merge(mut self, other: Imports) -> Imports {
        for (k, v) in other.0 {
            self.set(k, v);
        }
        self
    }
    fn values(self) -> Vec<Vec<u8>> {
        self.0.into_iter().map(|(_, v)| v).collect()
    }
}

fn lower(s: &[u8], case_sensitive: bool) -> Vec<u8> {
    if case_sensitive {
        s.to_vec()
    } else {
        strtolower(s)
    }
}

impl GlobalNamespaceImportFixer {
    fn import_constants(
        tokens: &mut Tokens,
        use_declarations: &[NamespaceUseAnalysis],
    ) -> Vec<Vec<u8>> {
        let (global, mut other) =
            Self::filter_use_declarations(use_declarations, |d| d.is_constant(), true);
        let count = tokens.count();
        let mut index = 0;
        while index < count {
            let token = &tokens[index];
            if token.is_classy() {
                index = tokens
                    .get_next_token_of_kind(index, &["{".into()], true)
                    .unwrap();
                index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index) + 1;
                continue;
            }
            if !token.is_kind(T_CONST) {
                index += 1;
                continue;
            }
            index = tokens.get_next_meaningful_token(index).unwrap();
            other.insert(tokens[index].get_content().to_vec());
            index += 1;
        }
        let mut indices = Vec::new();
        for index in (0..tokens.count()).rev() {
            let token = &tokens[index];
            if !token.is_kind(T_STRING) {
                continue;
            }
            let name = token.get_content().to_vec();
            if other.contains(&name) {
                continue;
            }
            if !is_constant_invocation(tokens, index) {
                continue;
            }
            let ns_separator_index = tokens.get_prev_meaningful_token(index).unwrap();
            if !tokens[ns_separator_index].is_kind(T_NS_SEPARATOR) {
                if !global.contains_key(&name) {
                    other.insert(name);
                }
                continue;
            }
            let prev_index = tokens
                .get_prev_meaningful_token(ns_separator_index)
                .unwrap();
            if tokens[prev_index].is_given_kind(&[CT_NAMESPACE_OPERATOR, T_STRING]) {
                continue;
            }
            indices.push(index);
        }
        Self::prepare_imports(tokens, &indices, &global, &other, true).values()
    }

    fn import_functions(
        tokens: &mut Tokens,
        use_declarations: &[NamespaceUseAnalysis],
    ) -> Vec<Vec<u8>> {
        let (global, mut other) =
            Self::filter_use_declarations(use_declarations, |d| d.is_function(), false);
        let mut declared = Vec::new();
        Self::find_function_declarations(tokens, 0, tokens.count() - 1, &mut declared);
        for name in declared {
            other.insert(strtolower(&name));
        }
        let analyzer = FunctionsAnalyzer::new();
        let mut indices = Vec::new();
        for index in (0..tokens.count()).rev() {
            let token = &tokens[index];
            if !token.is_kind(T_STRING) {
                continue;
            }
            let name = strtolower(token.get_content());
            if other.contains(&name) {
                continue;
            }
            if !analyzer.is_global_function_usage(tokens, index) {
                continue;
            }
            let ns_separator_index = tokens.get_prev_meaningful_token(index).unwrap();
            if !tokens[ns_separator_index].is_kind(T_NS_SEPARATOR) {
                if !global.contains_key(&name) {
                    other.insert(name);
                }
                continue;
            }
            indices.push(index);
        }
        Self::prepare_imports(tokens, &indices, &global, &other, false).values()
    }

    fn import_classes(
        tokens: &mut Tokens,
        use_declarations: &[NamespaceUseAnalysis],
    ) -> Vec<Vec<u8>> {
        let (global, mut other) =
            Self::filter_use_declarations(use_declarations, |d| d.is_class(), false);
        let mut doc_blocks: Vec<(usize, DocBlock)> = Vec::new();
        let count = tokens.count();
        let mut index = 0;
        while index < count {
            let token = &tokens[index];
            if token.is_kind(T_DOC_COMMENT) {
                let doc = DocBlock::new(token.get_content());
                Self::traverse_doc_block_types(&doc, |ty| {
                    if ty.contains(&b'\\') {
                        return None;
                    }
                    let name = strtolower(ty);
                    if !global.contains_key(&name) {
                        other.insert(name);
                    }
                    None
                });
                doc_blocks.push((index, doc));
            }
            if !tokens[index].is_classy() {
                index += 1;
                continue;
            }
            index = tokens.get_next_meaningful_token(index).unwrap();
            if tokens[index].is_kind(T_STRING) {
                other.insert(strtolower(tokens[index].get_content()));
            }
            index += 1;
        }
        let analyzer = ClassyAnalyzer;
        let mut indices = Vec::new();
        for index in (0..tokens.count()).rev() {
            let token = &tokens[index];
            if !token.is_kind(T_STRING) {
                continue;
            }
            let name = strtolower(token.get_content());
            if other.contains(&name) {
                continue;
            }
            if !analyzer.is_classy_invocation(tokens, index) {
                continue;
            }
            let ns_separator_index = tokens.get_prev_meaningful_token(index).unwrap();
            if !tokens[ns_separator_index].is_kind(T_NS_SEPARATOR) {
                if !global.contains_key(&name) {
                    other.insert(name);
                }
                continue;
            }
            if tokens[tokens
                .get_prev_meaningful_token(ns_separator_index)
                .unwrap()]
            .is_given_kind(&[CT_NAMESPACE_OPERATOR, T_STRING])
            {
                continue;
            }
            indices.push(index);
        }
        let mut imports = Imports::default();
        for (index, doc_block) in &doc_blocks {
            let changed = Self::traverse_doc_block_types(doc_block, |ty| {
                if ty[0] != b'\\' {
                    return Some(ty.to_vec());
                }
                let name = ty[1..].to_vec();
                let check_name = strtolower(&name);
                if check_name.contains(&b'\\') || other.contains(&check_name) {
                    return Some(ty.to_vec());
                }
                if let Some(g) = global.get(&check_name) {
                    return Some(g.clone().unwrap_or(name));
                }
                imports.set(check_name, name.clone());
                Some(name)
            });
            if changed {
                tokens.set(*index, Token::new(T_DOC_COMMENT, doc_block.get_content()));
            }
        }
        imports
            .merge(Self::prepare_imports(
                tokens, &indices, &global, &other, false,
            ))
            .values()
    }

    /// Removes the leading slash at the given indices (when the name is not already used).
    fn prepare_imports(
        tokens: &mut Tokens,
        indices: &[usize],
        global: &Global,
        other: &Other,
        case_sensitive: bool,
    ) -> Imports {
        let mut imports = Imports::default();
        for &index in indices {
            let name = tokens[index].get_content().to_vec();
            let check_name = lower(&name, case_sensitive);
            if other.contains(&check_name) {
                continue;
            }
            match global.get(&check_name) {
                None => imports.set(check_name, name),
                Some(Some(alias)) => tokens.set(index, Token::new(T_STRING, alias.clone())),
                Some(None) => {}
            }
            let p = tokens.get_prev_meaningful_token(index).unwrap();
            tokens.clear_at(p);
        }
        imports
    }

    fn fully_qualify_constants(tokens: &mut Tokens, use_declarations: &[NamespaceUseAnalysis]) {
        if !tokens.is_token_kind_found(CT_CONST_IMPORT) {
            return;
        }
        let (global, _) = Self::filter_use_declarations(
            use_declarations,
            |d| d.is_constant() && !d.is_aliased,
            true,
        );
        if global.is_empty() {
            return;
        }
        for index in (0..tokens.count()).rev() {
            let token = &tokens[index];
            if !token.is_kind(T_STRING) {
                continue;
            }
            if !global.contains_key(token.get_content()) {
                continue;
            }
            if tokens[tokens.get_prev_meaningful_token(index).unwrap()].is_kind(T_NS_SEPARATOR) {
                continue;
            }
            if !is_constant_invocation(tokens, index) {
                continue;
            }
            tokens.insert_at(index, vec![Token::new(T_NS_SEPARATOR, "\\")]);
        }
    }

    fn fully_qualify_functions(tokens: &mut Tokens, use_declarations: &[NamespaceUseAnalysis]) {
        if !tokens.is_token_kind_found(CT_FUNCTION_IMPORT) {
            return;
        }
        let (global, _) = Self::filter_use_declarations(
            use_declarations,
            |d| d.is_function() && !d.is_aliased,
            false,
        );
        if global.is_empty() {
            return;
        }
        let analyzer = FunctionsAnalyzer::new();
        for index in (0..tokens.count()).rev() {
            let token = &tokens[index];
            if !token.is_kind(T_STRING) {
                continue;
            }
            if !global.contains_key(&strtolower(token.get_content())) {
                continue;
            }
            if tokens[tokens.get_prev_meaningful_token(index).unwrap()].is_kind(T_NS_SEPARATOR) {
                continue;
            }
            if !analyzer.is_global_function_usage(tokens, index) {
                continue;
            }
            tokens.insert_at(index, vec![Token::new(T_NS_SEPARATOR, "\\")]);
        }
    }

    fn fully_qualify_classes(tokens: &mut Tokens, use_declarations: &[NamespaceUseAnalysis]) {
        if !tokens.is_token_kind_found(T_USE) {
            return;
        }
        let (global, _) = Self::filter_use_declarations(
            use_declarations,
            |d| d.is_class() && !d.is_aliased,
            false,
        );
        if global.is_empty() {
            return;
        }
        let analyzer = ClassyAnalyzer;
        for index in (0..tokens.count()).rev() {
            let token = &tokens[index];
            if token.is_kind(T_DOC_COMMENT) {
                let doc = DocBlock::new(token.get_content());
                let changed = Self::traverse_doc_block_types(&doc, |ty| {
                    if !global.contains_key(&strtolower(ty)) {
                        return Some(ty.to_vec());
                    }
                    Some([b"\\", ty].concat())
                });
                if changed {
                    tokens.set(index, Token::new(T_DOC_COMMENT, doc.get_content()));
                }
                continue;
            }
            if !token.is_kind(T_STRING) {
                continue;
            }
            if !global.contains_key(&strtolower(token.get_content())) {
                continue;
            }
            if tokens[tokens.get_prev_meaningful_token(index).unwrap()].is_kind(T_NS_SEPARATOR) {
                continue;
            }
            if !analyzer.is_classy_invocation(tokens, index) {
                continue;
            }
            tokens.insert_at(index, vec![Token::new(T_NS_SEPARATOR, "\\")]);
        }
    }

    fn filter_use_declarations(
        declarations: &[NamespaceUseAnalysis],
        callback: impl Fn(&NamespaceUseAnalysis) -> bool,
        case_sensitive: bool,
    ) -> (Global, Other) {
        let mut global = Global::new();
        let mut other = Other::new();
        for declaration in declarations {
            if !callback(declaration) {
                continue;
            }
            let full_name = ltrim_chars(&declaration.full_name, b"\\");
            if full_name.contains(&b'\\') {
                other.insert(lower(&declaration.short_name, case_sensitive));
                continue;
            }
            let check_name = lower(full_name, case_sensitive);
            let alias = &declaration.short_name;
            global.insert(
                check_name,
                if alias.as_slice() == full_name {
                    None
                } else {
                    Some(alias.clone())
                },
            );
        }
        (global, other)
    }

    /// Generator `findFunctionDeclarations`, collecting into `out`. Upstream's inner class loop reuses
    /// the outer `$index`, so the token right after a class body is skipped.
    fn find_function_declarations(
        tokens: &Tokens,
        start: usize,
        end: usize,
        out: &mut Vec<Vec<u8>>,
    ) {
        let mut index = start;
        while index <= end {
            let token = &tokens[index];
            if token.is_classy() {
                let class_start = tokens
                    .get_next_token_of_kind(index, &["{".into()], true)
                    .unwrap();
                let class_end = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, class_start);
                index = class_start;
                while index <= class_end {
                    if !tokens[index].is_kind(T_FUNCTION) {
                        index += 1;
                        continue;
                    }
                    let method_start = tokens
                        .get_next_token_of_kind(index, &["{".into(), ";".into()], true)
                        .unwrap();
                    if tokens[method_start].equals(";", true) {
                        index = method_start + 1;
                        continue;
                    }
                    let method_end = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, method_start);
                    Self::find_function_declarations(tokens, method_start, method_end, out);
                    index = method_end + 1;
                }
                index += 1;
                continue;
            }
            if !token.is_kind(T_FUNCTION) {
                index += 1;
                continue;
            }
            index = tokens.get_next_meaningful_token(index).unwrap();
            if tokens[index].is_kind(CT_RETURN_REF) {
                index = tokens.get_next_meaningful_token(index).unwrap();
            }
            if tokens[index].is_kind(T_STRING) {
                out.push(tokens[index].get_content().to_vec());
            }
            index += 1;
        }
    }

    fn traverse_doc_block_types(
        doc: &DocBlock,
        mut callback: impl FnMut(&[u8]) -> Option<Vec<u8>>,
    ) -> bool {
        let annotations = doc.get_annotations_of_type(TAGS_WITH_TYPES);
        if annotations.is_empty() {
            return false;
        }
        let mut changed = false;
        for annotation in annotations {
            let types = annotation.get_types();
            let new: Vec<Vec<u8>> = types
                .iter()
                .map(|full_type| {
                    let mut full_type = full_type.clone();
                    let source = full_type.clone();
                    let matches: Vec<(Vec<u8>, usize)> =
                        Preg::match_all("/[\\\\\\w]+(?![\\\\\\w:])/", &source, PREG_OFFSET_CAPTURE)
                            .iter()
                            .map(|m| (m.str(0).to_vec(), m.offset(0) as usize))
                            .collect();
                    for (ty, offset) in matches.into_iter().rev() {
                        if let Some(new_type) = callback(&ty)
                            && ty != new_type
                        {
                            full_type.splice(offset..offset + ty.len(), new_type);
                        }
                    }
                    full_type
                })
                .collect();
            if types != new {
                annotation.set_types(&new);
                changed = true;
            }
        }
        changed
    }
}

impl Fixer for GlobalNamespaceImportFixer {
    fn name(&self) -> &'static str {
        "global_namespace_import"
    }
    fn priority(&self) -> i32 {
        // Must run before NoUnusedImportsFixer, OrderedImportsFixer, StatementIndentationFixer.
        // Must run after NativeConstantInvocationFixer, NativeFunctionInvocationFixer, StringableForToStringFixer.
        0
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(
            self.name(),
            options,
            &["import_constants", "import_functions", "import_classes"],
        )?;
        self.import_constants = r.opt_bool("import_constants", None)?;
        self.import_functions = r.opt_bool("import_functions", None)?;
        self.import_classes = r.opt_bool("import_classes", Some(true))?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_DOC_COMMENT, T_NS_SEPARATOR, T_USE])
            && tokens.is_token_kind_found(T_NAMESPACE)
            && tokens.count_token_kind(T_NAMESPACE) == 1
            && tokens.is_monolithic_php()
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let namespace_analyses = NamespacesAnalyzer.get_declarations(tokens);
        if namespace_analyses.len() != 1 || namespace_analyses[0].is_global_namespace() {
            return;
        }
        let use_declarations = NamespaceUsesAnalyzer.get_declarations_from_tokens(tokens, false);
        let mut new_imports: Vec<(ImportType, Vec<Vec<u8>>)> = Vec::new();
        match self.import_constants {
            Some(true) => {
                new_imports.push(("const", Self::import_constants(tokens, &use_declarations)))
            }
            Some(false) => Self::fully_qualify_constants(tokens, &use_declarations),
            None => {}
        }
        match self.import_functions {
            Some(true) => new_imports.push((
                "function",
                Self::import_functions(tokens, &use_declarations),
            )),
            Some(false) => Self::fully_qualify_functions(tokens, &use_declarations),
            None => {}
        }
        match self.import_classes {
            Some(true) => {
                new_imports.push(("class", Self::import_classes(tokens, &use_declarations)))
            }
            Some(false) => Self::fully_qualify_classes(tokens, &use_declarations),
            None => {}
        }
        if !new_imports.is_empty() {
            let at_index = match use_declarations.last() {
                Some(u) => u.end_index + 1,
                None => NamespacesAnalyzer.get_declarations(tokens)[0].end_index + 1,
            };
            ImportProcessor::new(self.whitespaces_config.clone()).insert_imports(
                tokens,
                &new_imports,
                at_index,
            );
        }
    }
}
