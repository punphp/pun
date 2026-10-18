//! Port of `PhpCsFixer\Tokenizer\Tokens`: same method names (snake_cased), same semantics,
//! including the found-kinds registry and the block-edge cache, since both are observable
//! by fixers. Not ported: the static code-hash cache (pure optimization) and JSON helpers.

use std::any::Any;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::ops::Index;
use std::sync::Arc;

use crate::kind::Kind::{self, *};
use crate::token::{FoundKey, Proto, Token};

/// `Tokens::BLOCK_TYPE_*`; the discriminant is upstream's number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BlockType {
    Parenthesis = 1,
    Brace,
    IndexBracket,
    ArrayBracket,
    DynamicPropBrace,
    DynamicVarBrace,
    IndexBrace,
    GroupImportBrace,
    DestructuringBracket,
    ClassInstantiationParenthesis,
    Attribute,
    DisjunctiveNormalFormTypeParenthesis,
    DynamicClassConstantFetchBrace,
    ComplexStringVariable,
    PropertyHook,
}

impl BlockType {
    /// Every block type, in `getBlockEdgeDefinitions()` order.
    pub const ALL: [BlockType; 15] = [
        BlockType::Parenthesis,
        BlockType::Brace,
        BlockType::IndexBracket,
        BlockType::ArrayBracket,
        BlockType::DynamicPropBrace,
        BlockType::DynamicVarBrace,
        BlockType::IndexBrace,
        BlockType::GroupImportBrace,
        BlockType::DestructuringBracket,
        BlockType::ClassInstantiationParenthesis,
        BlockType::Attribute,
        BlockType::DisjunctiveNormalFormTypeParenthesis,
        BlockType::DynamicClassConstantFetchBrace,
        BlockType::ComplexStringVariable,
        BlockType::PropertyHook,
    ];
}

#[derive(Clone, Debug, Default)]
struct BlockCache {
    start: HashMap<usize, usize>,
    end: HashMap<usize, usize>,
}

#[derive(Clone, Debug)]
pub struct Tokens {
    tokens: Vec<Token>,
    found_kinds: Vec<i64>,
    found_contents: HashMap<Vec<u8>, i64>,
    /// `blockStartCache` / `blockEndCache`; interior mutability so `find_block_end` takes `&self`.
    blocks: RefCell<BlockCache>,
    /// `$namespaceDeclarations`: NamespacesAnalyzer's result (a pun-fixers type, hence `Any`), dropped on every change.
    namespace_declarations: RefCell<Option<Arc<dyn Any + Send + Sync>>>,
    changed: bool,
}

impl Default for Tokens {
    /// `new Tokens()`: empty, with a found-kinds slot for every kind.
    fn default() -> Self {
        Tokens {
            tokens: Vec::new(),
            found_kinds: vec![0; Kind::ALL.len()],
            found_contents: HashMap::new(),
            blocks: RefCell::default(),
            namespace_declarations: RefCell::default(),
            changed: false,
        }
    }
}

impl Index<usize> for Tokens {
    type Output = Token;
    fn index(&self, i: usize) -> &Token {
        &self.tokens[i]
    }
}

impl Tokens {
    pub const BLOCK_TYPE_PARENTHESIS: BlockType = BlockType::Parenthesis;
    pub const BLOCK_TYPE_BRACE: BlockType = BlockType::Brace;
    pub const BLOCK_TYPE_INDEX_BRACKET: BlockType = BlockType::IndexBracket;
    pub const BLOCK_TYPE_ARRAY_BRACKET: BlockType = BlockType::ArrayBracket;
    pub const BLOCK_TYPE_DYNAMIC_PROP_BRACE: BlockType = BlockType::DynamicPropBrace;
    pub const BLOCK_TYPE_DYNAMIC_VAR_BRACE: BlockType = BlockType::DynamicVarBrace;
    pub const BLOCK_TYPE_INDEX_BRACE: BlockType = BlockType::IndexBrace;
    pub const BLOCK_TYPE_GROUP_IMPORT_BRACE: BlockType = BlockType::GroupImportBrace;
    pub const BLOCK_TYPE_DESTRUCTURING_BRACKET: BlockType = BlockType::DestructuringBracket;
    pub const BLOCK_TYPE_CLASS_INSTANTIATION_PARENTHESIS: BlockType =
        BlockType::ClassInstantiationParenthesis;
    pub const BLOCK_TYPE_ATTRIBUTE: BlockType = BlockType::Attribute;
    pub const BLOCK_TYPE_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS: BlockType =
        BlockType::DisjunctiveNormalFormTypeParenthesis;
    pub const BLOCK_TYPE_DYNAMIC_CLASS_CONSTANT_FETCH_BRACE: BlockType =
        BlockType::DynamicClassConstantFetchBrace;
    pub const BLOCK_TYPE_COMPLEX_STRING_VARIABLE: BlockType = BlockType::ComplexStringVariable;
    pub const BLOCK_TYPE_PROPERTY_HOOK: BlockType = BlockType::PropertyHook;
    // deprecated upstream aliases
    pub const BLOCK_TYPE_PARENTHESIS_BRACE: BlockType = Self::BLOCK_TYPE_PARENTHESIS;
    pub const BLOCK_TYPE_CURLY_BRACE: BlockType = Self::BLOCK_TYPE_BRACE;
    pub const BLOCK_TYPE_INDEX_SQUARE_BRACE: BlockType = Self::BLOCK_TYPE_INDEX_BRACKET;
    pub const BLOCK_TYPE_ARRAY_SQUARE_BRACE: BlockType = Self::BLOCK_TYPE_ARRAY_BRACKET;
    pub const BLOCK_TYPE_ARRAY_INDEX_CURLY_BRACE: BlockType = Self::BLOCK_TYPE_INDEX_BRACE;
    pub const BLOCK_TYPE_DESTRUCTURING_SQUARE_BRACE: BlockType =
        Self::BLOCK_TYPE_DESTRUCTURING_BRACKET;
    pub const BLOCK_TYPE_BRACE_CLASS_INSTANTIATION: BlockType =
        Self::BLOCK_TYPE_CLASS_INSTANTIATION_PARENTHESIS;
    pub const BLOCK_TYPE_DYNAMIC_CLASS_CONSTANT_FETCH_CURLY_BRACE: BlockType =
        Self::BLOCK_TYPE_DYNAMIC_CLASS_CONSTANT_FETCH_BRACE;

    /// `getBlockEdgeDefinitions()[$type]`: (start, end).
    pub fn block_edges(t: BlockType) -> (Proto<'static>, Proto<'static>) {
        use Proto::{Char as C, KindContent as K};
        match t {
            BlockType::Parenthesis => (C(b"("), C(b")")),
            BlockType::Brace => (C(b"{"), C(b"}")),
            BlockType::IndexBracket => (C(b"["), C(b"]")),
            BlockType::ArrayBracket => (
                K(CT_ARRAY_BRACKET_OPEN, b"["),
                K(CT_ARRAY_BRACKET_CLOSE, b"]"),
            ),
            BlockType::DynamicPropBrace => (
                K(CT_DYNAMIC_PROP_BRACE_OPEN, b"{"),
                K(CT_DYNAMIC_PROP_BRACE_CLOSE, b"}"),
            ),
            BlockType::DynamicVarBrace => (
                K(CT_DYNAMIC_VAR_BRACE_OPEN, b"{"),
                K(CT_DYNAMIC_VAR_BRACE_CLOSE, b"}"),
            ),
            BlockType::IndexBrace => (
                K(CT_ARRAY_INDEX_BRACE_OPEN, b"{"),
                K(CT_ARRAY_INDEX_BRACE_CLOSE, b"}"),
            ),
            BlockType::GroupImportBrace => (
                K(CT_GROUP_IMPORT_BRACE_OPEN, b"{"),
                K(CT_GROUP_IMPORT_BRACE_CLOSE, b"}"),
            ),
            BlockType::DestructuringBracket => (
                K(CT_DESTRUCTURING_BRACKET_OPEN, b"["),
                K(CT_DESTRUCTURING_BRACKET_CLOSE, b"]"),
            ),
            BlockType::ClassInstantiationParenthesis => (
                K(CT_CLASS_INSTANTIATION_PARENTHESIS_OPEN, b"("),
                K(CT_CLASS_INSTANTIATION_PARENTHESIS_CLOSE, b")"),
            ),
            BlockType::Attribute => (K(T_ATTRIBUTE, b"#["), K(CT_ATTRIBUTE_CLOSE, b"]")),
            BlockType::DisjunctiveNormalFormTypeParenthesis => (
                K(CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_OPEN, b"("),
                K(CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_CLOSE, b")"),
            ),
            BlockType::DynamicClassConstantFetchBrace => (
                K(CT_DYNAMIC_CLASS_CONSTANT_FETCH_BRACE_OPEN, b"{"),
                K(CT_DYNAMIC_CLASS_CONSTANT_FETCH_BRACE_CLOSE, b"}"),
            ),
            BlockType::ComplexStringVariable => (
                K(T_DOLLAR_OPEN_CURLY_BRACES, b"${"),
                K(CT_DOLLAR_CLOSE_CURLY_BRACES, b"}"),
            ),
            BlockType::PropertyHook => (
                K(CT_PROPERTY_HOOK_BRACE_OPEN, b"{"),
                K(CT_PROPERTY_HOOK_BRACE_CLOSE, b"}"),
            ),
        }
    }

    /// `detectBlockType`: (type, is_start) for a block edge token. Matches upstream's
    /// lookup by kind (array tokens) or content (others).
    pub fn detect_block_type(token: &Token) -> Option<(BlockType, bool)> {
        use BlockType as B;
        Some(match token.id {
            None => match &token.content[..] {
                b"(" => (B::Parenthesis, true),
                b")" => (B::Parenthesis, false),
                b"{" => (B::Brace, true),
                b"}" => (B::Brace, false),
                b"[" => (B::IndexBracket, true),
                b"]" => (B::IndexBracket, false),
                _ => return None,
            },
            Some(id) => match id {
                CT_ARRAY_BRACKET_OPEN => (B::ArrayBracket, true),
                CT_ARRAY_BRACKET_CLOSE => (B::ArrayBracket, false),
                CT_DYNAMIC_PROP_BRACE_OPEN => (B::DynamicPropBrace, true),
                CT_DYNAMIC_PROP_BRACE_CLOSE => (B::DynamicPropBrace, false),
                CT_DYNAMIC_VAR_BRACE_OPEN => (B::DynamicVarBrace, true),
                CT_DYNAMIC_VAR_BRACE_CLOSE => (B::DynamicVarBrace, false),
                CT_ARRAY_INDEX_BRACE_OPEN => (B::IndexBrace, true),
                CT_ARRAY_INDEX_BRACE_CLOSE => (B::IndexBrace, false),
                CT_GROUP_IMPORT_BRACE_OPEN => (B::GroupImportBrace, true),
                CT_GROUP_IMPORT_BRACE_CLOSE => (B::GroupImportBrace, false),
                CT_DESTRUCTURING_BRACKET_OPEN => (B::DestructuringBracket, true),
                CT_DESTRUCTURING_BRACKET_CLOSE => (B::DestructuringBracket, false),
                CT_CLASS_INSTANTIATION_PARENTHESIS_OPEN => (B::ClassInstantiationParenthesis, true),
                CT_CLASS_INSTANTIATION_PARENTHESIS_CLOSE => {
                    (B::ClassInstantiationParenthesis, false)
                }
                T_ATTRIBUTE => (B::Attribute, true),
                CT_ATTRIBUTE_CLOSE => (B::Attribute, false),
                CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_OPEN => {
                    (B::DisjunctiveNormalFormTypeParenthesis, true)
                }
                CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_CLOSE => {
                    (B::DisjunctiveNormalFormTypeParenthesis, false)
                }
                CT_DYNAMIC_CLASS_CONSTANT_FETCH_BRACE_OPEN => {
                    (B::DynamicClassConstantFetchBrace, true)
                }
                CT_DYNAMIC_CLASS_CONSTANT_FETCH_BRACE_CLOSE => {
                    (B::DynamicClassConstantFetchBrace, false)
                }
                T_DOLLAR_OPEN_CURLY_BRACES => (B::ComplexStringVariable, true),
                CT_DOLLAR_CLOSE_CURLY_BRACES => (B::ComplexStringVariable, false),
                CT_PROPERTY_HOOK_BRACE_OPEN => (B::PropertyHook, true),
                CT_PROPERTY_HOOK_BRACE_CLOSE => (B::PropertyHook, false),
                _ => return None,
            },
        })
    }

    pub fn from_array(tokens: Vec<Token>) -> Tokens {
        let mut t = Tokens::default();
        for tok in &tokens {
            t.register_found_token(tok);
        }
        t.tokens = tokens;
        t.clear_changed();
        t
    }

    /// `Tokens::fromCode`: `token_get_all($code, TOKEN_PARSE)` + transformers.
    pub fn from_code(code: &[u8]) -> Tokens {
        let mut t = Tokens::default();
        t.set_code(code);
        t.clear_changed();
        t
    }

    pub fn set_code(&mut self, code: &[u8]) {
        if !self.tokens.is_empty() && code == self.generate_code() {
            return;
        }
        let mut raw = crate::lexer::tokenize(code);
        crate::parse_mode::apply_parse_mode(&mut raw);
        self.tokens.clear();
        self.found_kinds = vec![0; Kind::ALL.len()];
        self.found_contents.clear();
        for r in &raw {
            let tok = Token {
                id: r.kind,
                content: r.text.to_vec(),
            };
            self.register_found_token(&tok);
            self.tokens.push(tok);
        }
        crate::transformers::transform(self);
        self.changed = true;
        *self.namespace_declarations.get_mut() = None;
        *self.blocks.get_mut() = BlockCache::default();
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }
    pub fn count(&self) -> usize {
        self.tokens.len()
    }
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Token> {
        self.tokens.iter()
    }
    pub fn as_slice(&self) -> &[Token] {
        &self.tokens
    }
    pub fn offset_exists(&self, index: isize) -> bool {
        index >= 0 && (index as usize) < self.tokens.len()
    }

    fn invalidate_block_cache_at(&mut self, index: usize) {
        let c = self.blocks.get_mut();
        if let Some(end) = c.start.remove(&index) {
            c.end.remove(&end);
        }
        if let Some(start) = c.end.remove(&index) {
            c.start.remove(&start);
        }
    }

    /// `$tokens[$index] = $token` (offsetSet).
    pub fn set(&mut self, index: usize, token: Token) {
        self.invalidate_block_cache_at(index);
        if !self.tokens[index].equals_token(&token, true) {
            let old = std::mem::replace(&mut self.tokens[index], Token::char(""));
            self.unregister_found_token(&old);
            self.changed = true;
            *self.namespace_declarations.get_mut() = None;
            self.register_found_token(&token);
        }
        self.tokens[index] = token;
    }

    pub fn clear_changed(&mut self) {
        self.changed = false;
    }

    pub fn is_changed(&self) -> bool {
        self.changed
    }

    /// `getNamespaceDeclarations()`: `compute` (NamespacesAnalyzer::getDeclarations) runs once per token-list version;
    /// every edit (set to a different token, insert, clear_empty_tokens, set_code) drops the cached value.
    pub fn namespace_declarations<T: Any + Send + Sync>(
        &self,
        compute: impl FnOnce(&Tokens) -> T,
    ) -> Arc<T> {
        if let Some(v) = self
            .namespace_declarations
            .borrow()
            .as_ref()
            .and_then(|v| v.clone().downcast::<T>().ok())
        {
            return v;
        }
        let v = Arc::new(compute(self));
        *self.namespace_declarations.borrow_mut() = Some(v.clone());
        v
    }

    pub fn clear_empty_tokens(&mut self) {
        if !self.is_token_kind_found(FoundKey::Content(b"")) {
            return;
        }
        let limit = self.tokens.len();
        let mut index = (0..limit).find(|&i| self.is_empty_at(i)).unwrap_or(limit);
        let mut count = index;
        while index < limit {
            if !self.is_empty_at(index) {
                self.tokens.swap(count, index);
                let c = self.blocks.get_mut();
                if let Some(other) = c.start.remove(&index) {
                    c.start.insert(count, other);
                    c.end.insert(other, count);
                }
                if let Some(other) = c.end.remove(&index) {
                    c.start.insert(other, count);
                    c.end.insert(count, other);
                }
                count += 1;
            }
            index += 1;
        }
        // Tokens moved: index-based caches go (upstream resets namespaceDeclarations here too).
        *self.namespace_declarations.get_mut() = None;
        self.found_contents.insert(Vec::new(), 0);
        // updateSizeByTrimmingTrailingEmptyTokens
        while self
            .tokens
            .last()
            .is_some_and(|t| t.id.is_none() && t.content.is_empty())
        {
            self.tokens.pop();
        }
    }

    pub fn ensure_whitespace_at_index(
        &mut self,
        index: usize,
        index_offset: usize,
        whitespace: &[u8],
    ) -> bool {
        let remove_last_comment_line =
            |tokens: &mut Tokens, index: usize, whitespace: &[u8]| -> Vec<u8> {
                let token = &tokens[index];
                if index_offset == 1 && token.is_kind(T_OPEN_TAG) {
                    let trimmed = crate::php_compat::rtrim(&token.content).to_vec();
                    if whitespace.starts_with(b"\r\n") {
                        tokens.set(
                            index,
                            Token::new(T_OPEN_TAG, [&trimmed[..], b"\r\n"].concat()),
                        );
                        return whitespace[2..].to_vec();
                    }
                    tokens.set(
                        index,
                        Token::new(T_OPEN_TAG, [&trimmed[..], &whitespace[..1]].concat()),
                    );
                    return whitespace[1..].to_vec();
                }
                whitespace.to_vec()
            };

        if self.tokens[index].is_whitespace(None) {
            let whitespace = remove_last_comment_line(self, index - 1, whitespace);
            if whitespace.is_empty() {
                self.clear_at(index);
            } else {
                self.set(index, Token::new(T_WHITESPACE, whitespace));
            }
            return false;
        }

        let whitespace = remove_last_comment_line(self, index, whitespace);
        if whitespace.is_empty() {
            return false;
        }
        self.insert_at(
            index + index_offset,
            vec![Token::new(T_WHITESPACE, whitespace)],
        );
        true
    }

    pub fn find_block_end(&self, t: BlockType, search_index: usize) -> usize {
        self.find_opposite_block_edge(t, search_index, true)
    }

    pub fn find_block_start(&self, t: BlockType, search_index: usize) -> usize {
        self.find_opposite_block_edge(t, search_index, false)
    }

    /// `findGivenKind($kind, $start, $end)` for one kind: indices in order.
    pub fn find_given_kind(&self, kind: Kind, start: usize, end: Option<usize>) -> Vec<usize> {
        self.find_given_kinds(&[kind], start, end)
            .remove(&kind)
            .unwrap_or_default()
    }

    /// `findGivenKind([...], $start, $end)`: kind => indices.
    pub fn find_given_kinds(
        &self,
        kinds: &[Kind],
        start: usize,
        end: Option<usize>,
    ) -> BTreeMap<Kind, Vec<usize>> {
        let end = end.unwrap_or(self.tokens.len());
        let mut out: BTreeMap<Kind, Vec<usize>> = kinds.iter().map(|&k| (k, Vec::new())).collect();
        let found: Vec<Kind> = kinds
            .iter()
            .copied()
            .filter(|&k| self.is_token_kind_found(k))
            .collect();
        if !found.is_empty() {
            for i in start..end {
                if let Some(id) = self.tokens[i].id
                    && found.contains(&id)
                {
                    out.get_mut(&id).unwrap().push(i);
                }
            }
        }
        out
    }

    pub fn generate_code(&self) -> Vec<u8> {
        if self.tokens.is_empty() {
            return Vec::new();
        }
        self.generate_partial_code(0, self.tokens.len() - 1)
    }

    /// Inclusive range, like upstream.
    pub fn generate_partial_code(&self, start: usize, end: usize) -> Vec<u8> {
        let mut code = Vec::new();
        for t in &self.tokens[start..=end] {
            code.extend_from_slice(&t.content);
        }
        code
    }

    fn step(&self, index: usize, direction: isize) -> Option<usize> {
        let i = index as isize + direction;
        self.offset_exists(i).then_some(i as usize)
    }

    pub fn get_next_non_whitespace(
        &self,
        index: usize,
        whitespaces: Option<&[u8]>,
    ) -> Option<usize> {
        self.get_non_whitespace_sibling(index, 1, whitespaces)
    }

    pub fn get_prev_non_whitespace(
        &self,
        index: usize,
        whitespaces: Option<&[u8]>,
    ) -> Option<usize> {
        self.get_non_whitespace_sibling(index, -1, whitespaces)
    }

    pub fn get_non_whitespace_sibling(
        &self,
        mut index: usize,
        direction: isize,
        whitespaces: Option<&[u8]>,
    ) -> Option<usize> {
        loop {
            index = self.step(index, direction)?;
            if !self.tokens[index].is_whitespace(whitespaces) {
                return Some(index);
            }
        }
    }

    pub fn get_next_token_of_kind(
        &self,
        index: usize,
        tokens: &[Proto<'_>],
        case_sensitive: bool,
    ) -> Option<usize> {
        self.get_token_of_kind_sibling(index, 1, tokens, case_sensitive)
    }

    pub fn get_prev_token_of_kind(
        &self,
        index: usize,
        tokens: &[Proto<'_>],
        case_sensitive: bool,
    ) -> Option<usize> {
        self.get_token_of_kind_sibling(index, -1, tokens, case_sensitive)
    }

    pub fn get_token_of_kind_sibling(
        &self,
        mut index: usize,
        direction: isize,
        tokens: &[Proto<'_>],
        case_sensitive: bool,
    ) -> Option<usize> {
        // Upstream drops the prototypes whose kind is not in the collection; skip the copy when all of them are.
        let filtered: Vec<Proto<'_>>;
        let tokens = if tokens
            .iter()
            .all(|t| self.is_token_kind_found(t.found_key()))
        {
            tokens
        } else {
            filtered = tokens
                .iter()
                .copied()
                .filter(|t| self.is_token_kind_found(t.found_key()))
                .collect();
            &filtered
        };
        if tokens.is_empty() {
            return None;
        }
        loop {
            index = self.step(index, direction)?;
            if self.tokens[index].equals_any(tokens, case_sensitive) {
                return Some(index);
            }
        }
    }

    pub fn get_token_not_of_kind_sibling(
        &self,
        index: usize,
        direction: isize,
        tokens: &[Proto<'_>],
    ) -> Option<usize> {
        self.get_token_not_of_kind(index, direction, |t| t.equals_any(tokens, true))
    }

    pub fn get_token_not_of_kinds_sibling(
        &self,
        index: usize,
        direction: isize,
        kinds: &[Kind],
    ) -> Option<usize> {
        self.get_token_not_of_kind(index, direction, |t| t.is_given_kind(kinds))
    }

    pub fn get_meaningful_token_sibling(&self, index: usize, direction: isize) -> Option<usize> {
        self.get_token_not_of_kind(index, direction, |t| {
            matches!(t.id, Some(T_WHITESPACE | T_COMMENT | T_DOC_COMMENT))
        })
    }

    pub fn get_non_empty_sibling(&self, mut index: usize, direction: isize) -> Option<usize> {
        loop {
            index = self.step(index, direction)?;
            if !self.is_empty_at(index) {
                return Some(index);
            }
        }
    }

    pub fn get_next_meaningful_token(&self, index: usize) -> Option<usize> {
        self.get_meaningful_token_sibling(index, 1)
    }

    pub fn get_prev_meaningful_token(&self, index: usize) -> Option<usize> {
        self.get_meaningful_token_sibling(index, -1)
    }

    /// `findSequence($sequence, $start, $end, $caseSensitive)`; returns matched indices.
    /// `case_sensitive(key)` gives the per-element flag (upstream's bool or array form).
    pub fn find_sequence(
        &self,
        sequence: &[Proto<'_>],
        start: usize,
        end: Option<usize>,
        case_sensitive: impl Fn(usize) -> bool,
    ) -> Option<Vec<usize>> {
        assert!(!sequence.is_empty(), "Invalid sequence.");
        if self.tokens.is_empty() {
            return None;
        }
        let end = end.map_or(self.tokens.len() - 1, |e| e.min(self.tokens.len() - 1));
        if start + sequence.len() - 1 > end {
            return None;
        }
        for (key, p) in sequence.iter().enumerate() {
            let k = match p {
                Proto::Kind(k) | Proto::KindContent(k, _) => Some(*k),
                Proto::Char(c) => {
                    assert!(
                        !c.is_empty(),
                        "Non-meaningful (empty) token at position: \"{key}\"."
                    );
                    None
                }
            };
            assert!(
                !matches!(k, Some(T_COMMENT | T_DOC_COMMENT | T_WHITESPACE)),
                "Non-meaningful token at position: \"{key}\"."
            );
            if let Proto::KindContent(_, c) = p {
                assert!(
                    !c.is_empty(),
                    "Non-meaningful (empty) token at position: \"{key}\"."
                );
            }
        }
        if !sequence
            .iter()
            .all(|t| self.is_token_kind_found(t.found_key()))
        {
            return None;
        }
        'outer: for index in start..=end {
            if !self.tokens[index].equals(sequence[0], case_sensitive(0)) {
                continue;
            }
            let mut result = vec![index];
            let mut curr = index;
            for (key, &tok) in sequence.iter().enumerate().skip(1) {
                match self.get_next_meaningful_token(curr) {
                    Some(c) if c <= end => curr = c,
                    _ => continue 'outer,
                }
                if !self.tokens[curr].equals(tok, case_sensitive(key)) {
                    continue 'outer;
                }
                result.push(curr);
            }
            return Some(result);
        }
        None
    }

    pub fn insert_at(&mut self, index: usize, items: Vec<Token>) {
        let mut slices = BTreeMap::new();
        slices.insert(index, items);
        self.insert_slices(slices);
    }

    /// `insertSlices([$index => [$tokens...], ...])`; indices refer to the collection before insertion.
    pub fn insert_slices(&mut self, slices: BTreeMap<usize, Vec<Token>>) {
        let count: usize = slices.values().map(Vec::len).sum();
        if count == 0 {
            return;
        }
        self.changed = true;
        *self.blocks.get_mut() = BlockCache::default();
        *self.namespace_declarations.get_mut() = None;
        let old_size = self.tokens.len();
        let far = *slices.keys().next_back().unwrap();
        assert!(
            far <= old_size,
            "Cannot insert index \"{far}\" outside of collection."
        );
        for t in slices.values().flatten() {
            assert!(
                !t.content.is_empty(),
                "Must not add empty token to collection."
            );
            self.register_found_token(t);
        }
        // Back to front so earlier indices stay valid.
        for (index, slice) in slices.into_iter().rev() {
            self.tokens.splice(index..index, slice);
        }
    }

    pub fn is_empty_at(&self, index: usize) -> bool {
        let t = &self.tokens[index];
        t.id.is_none() && t.content.is_empty()
    }

    pub fn clear_at(&mut self, index: usize) {
        self.set(index, Token::char(""));
    }

    pub fn override_range(&mut self, index_start: usize, index_end: usize, items: Vec<Token>) {
        let mut index_to_change = index_end + 1 - index_start;
        let items_count = items.len();
        if items_count > index_to_change {
            let mut placeholders = Vec::new();
            while items_count > index_to_change {
                placeholders.push(Token::char("__PLACEHOLDER__"));
                index_to_change += 1;
            }
            self.insert_at(index_end + 1, placeholders);
        }
        for (i, item) in items.into_iter().enumerate() {
            self.set(index_start + i, item);
        }
        if items_count < index_to_change {
            self.clear_range(index_start + items_count, index_end);
        }
    }

    pub fn remove_leading_whitespace(&mut self, index: usize, whitespaces: Option<&[u8]>) {
        self.remove_whitespace_safely(index, -1, whitespaces);
    }

    pub fn remove_trailing_whitespace(&mut self, index: usize, whitespaces: Option<&[u8]>) {
        self.remove_whitespace_safely(index, 1, whitespaces);
    }

    pub fn is_all_token_kinds_found(&self, kinds: &[Kind]) -> bool {
        kinds.iter().all(|&k| self.is_token_kind_found(k))
    }

    pub fn is_any_token_kinds_found(&self, kinds: &[Kind]) -> bool {
        kinds.iter().any(|&k| self.is_token_kind_found(k))
    }

    pub fn is_token_kind_found<'a>(&self, key: impl Into<FoundKey<'a>>) -> bool {
        self.count_token_kind(key) != 0
    }

    pub fn count_token_kind<'a>(&self, key: impl Into<FoundKey<'a>>) -> i64 {
        match key.into() {
            FoundKey::Kind(k) => self.found_kinds[k as usize],
            FoundKey::Content(c) => *self.found_contents.get(c).unwrap_or(&0),
        }
    }

    /// Inclusive range.
    pub fn clear_range(&mut self, index_start: usize, index_end: usize) {
        for i in index_start..=index_end {
            self.clear_at(i);
        }
    }

    pub fn is_monolithic_php(&self) -> bool {
        if self.count_token_kind(T_OPEN_TAG) + self.count_token_kind(T_OPEN_TAG_WITH_ECHO) != 1 {
            return false;
        }
        let html = self.count_token_kind(T_INLINE_HTML);
        // Preg::match('/^#!.+$/', $this[0]->getContent()): `.` excludes "\n", `$` allows one trailing "\n".
        let shebang = |c: &[u8]| {
            let c = c.strip_suffix(b"\n").unwrap_or(c);
            c.len() > 2 && c.starts_with(b"#!") && !c.contains(&b'\n')
        };
        html == 0 || (html == 1 && shebang(&self.tokens[0].content))
    }

    pub fn is_partial_code_multiline(&self, start: usize, end: usize) -> bool {
        self.tokens[start..=end]
            .iter()
            .any(|t| t.content.contains(&b'\n'))
    }

    pub fn has_alternative_syntax(&self) -> bool {
        self.is_any_token_kinds_found(&[
            T_ENDDECLARE,
            T_ENDFOR,
            T_ENDFOREACH,
            T_ENDIF,
            T_ENDSWITCH,
            T_ENDWHILE,
        ])
    }

    pub fn clear_token_and_merge_surrounding_whitespace(&mut self, index: usize) {
        let count = self.tokens.len();
        self.clear_at(index);
        if index == count - 1 {
            return;
        }
        let Some(next) = self.get_non_empty_sibling(index, 1) else {
            return;
        };
        if !self.tokens[next].is_whitespace(None) {
            return;
        }
        let prev = self.get_non_empty_sibling(index, -1).expect("prev index");
        if self.tokens[prev].is_whitespace(None) {
            let merged = [
                &self.tokens[prev].content[..],
                &self.tokens[next].content[..],
            ]
            .concat();
            self.set(prev, Token::new(T_WHITESPACE, merged));
        } else if self.is_empty_at(prev + 1) {
            let c = self.tokens[next].content.clone();
            self.set(prev + 1, Token::new(T_WHITESPACE, c));
        }
        self.clear_at(next);
    }

    fn remove_whitespace_safely(
        &mut self,
        index: usize,
        direction: isize,
        whitespaces: Option<&[u8]>,
    ) {
        // isset($this[null]) throws upstream when there is no sibling.
        let wi = self
            .get_non_empty_sibling(index, direction)
            .expect("Cannot access offset of type null on SplFixedArray");
        if !self.tokens[wi].is_whitespace(None) {
            return;
        }
        let mut new_content: Vec<u8> = Vec::new();
        let mut to_check = self.tokens[wi].clone();
        if wi > 0
            && self.tokens[wi - 1].is_comment()
            && !self.tokens[wi - 1].content.starts_with(b"/*")
        {
            // Preg::split('/^(\R)/', ...): first line break vs the rest.
            let c = &self.tokens[wi].content;
            let nl = if c.starts_with(b"\r\n") {
                2
            } else if matches!(c.first(), Some(b'\n' | b'\r' | b'\x0b' | b'\x0c')) {
                1
            } else {
                // Preg::split found no leading line break: upstream builds a Token from null and throws.
                panic!("Undefined array key 1");
            };
            new_content = c[..nl].to_vec();
            let rest = c[nl..].to_vec();
            if rest.is_empty() {
                return;
            }
            to_check = Token::new(T_WHITESPACE, rest);
        }
        if !to_check.is_whitespace(whitespaces) {
            return;
        }
        if new_content.is_empty() {
            self.clear_at(wi);
        } else {
            self.set(wi, Token::new(T_WHITESPACE, new_content));
        }
    }

    fn find_opposite_block_edge(&self, t: BlockType, search_index: usize, find_end: bool) -> usize {
        let (mut start_edge, mut end_edge) = Self::block_edges(t);
        if find_end {
            if let Some(&e) = self.blocks.borrow().start.get(&search_index) {
                return e;
            }
        } else if let Some(&s) = self.blocks.borrow().end.get(&search_index) {
            return s;
        }
        let start_index = search_index;
        let (end_index, offset): (isize, isize) = if find_end {
            (self.tokens.len() as isize - 1, 1)
        } else {
            (0, -1)
        };
        if !find_end {
            std::mem::swap(&mut start_edge, &mut end_edge);
        }
        assert!(
            self.tokens[start_index].equals(start_edge, true),
            "Invalid param $startIndex - not a proper block \"{}\".",
            if find_end { "start" } else { "end" }
        );
        let mut level = 0;
        let mut index = start_index as isize;
        while index != end_index {
            let token = &self.tokens[index as usize];
            if token.equals(start_edge, true) {
                level += 1;
            } else if token.equals(end_edge, true) {
                level -= 1;
                if level == 0 {
                    break;
                }
            }
            index += offset;
        }
        let index = index as usize;
        assert!(
            self.tokens[index].equals(end_edge, true),
            "Missing block \"{}\".",
            if find_end { "end" } else { "start" }
        );
        let (a, b) = if start_index < index {
            (start_index, index)
        } else {
            (index, start_index)
        };
        let mut c = self.blocks.borrow_mut();
        c.start.insert(a, b);
        c.end.insert(b, a);
        index
    }

    fn register_found_token(&mut self, token: &Token) {
        self.bump(token, 1);
    }

    fn unregister_found_token(&mut self, token: &Token) {
        self.bump(token, -1);
    }

    fn bump(&mut self, token: &Token, by: i64) {
        match token.id {
            Some(k) => self.found_kinds[k as usize] += by,
            None => match self.found_contents.get_mut(&token.content[..]) {
                Some(n) => *n += by,
                None => {
                    self.found_contents.insert(token.content.clone(), by);
                }
            },
        }
    }

    fn get_token_not_of_kind(
        &self,
        mut index: usize,
        direction: isize,
        filter: impl Fn(&Token) -> bool,
    ) -> Option<usize> {
        loop {
            index = self.step(index, direction)?;
            if self.is_empty_at(index) || filter(&self.tokens[index]) {
                continue;
            }
            return Some(index);
        }
    }
}

impl<'a> IntoIterator for &'a Tokens {
    type Item = &'a Token;
    type IntoIter = std::slice::Iter<'a, Token>;
    fn into_iter(self) -> Self::IntoIter {
        self.tokens.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edit_ops() {
        let mut t = Tokens::from_code(b"<?php foo(1, [2]);");
        assert_eq!(t.generate_code(), b"<?php foo(1, [2]);");
        let open = t.get_next_token_of_kind(0, &["(".into()], true).unwrap();
        let close = t.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
        assert!(t[close].equals(")", true));
        assert_eq!(
            t.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, close),
            open
        );
        let comma = t.get_next_token_of_kind(open, &[",".into()], true).unwrap();
        t.ensure_whitespace_at_index(comma + 1, 0, b"  ");
        t.clear_at(open + 1);
        t.clear_empty_tokens();
        assert_eq!(t.generate_code(), b"<?php foo(,  [2]);");
        t.insert_at(open + 1, vec![Token::new(T_LNUMBER, "7")]);
        assert_eq!(t.generate_code(), b"<?php foo(7,  [2]);");
        t.remove_trailing_whitespace(comma, None);
        t.clear_empty_tokens();
        assert_eq!(t.generate_code(), b"<?php foo(7,[2]);");
        assert!(t.is_token_kind_found(CT_ARRAY_BRACKET_OPEN));
        assert_eq!(
            t.find_sequence(&[T_STRING.into(), "(".into()], 0, None, |_| true),
            Some(vec![1, 2])
        );
    }

    #[test]
    fn block_type_all_is_in_upstream_order() {
        // Tokens::BLOCK_TYPE_PARENTHESIS = 1 ... BLOCK_TYPE_PROPERTY_HOOK = 15.
        let numbers: Vec<u8> = BlockType::ALL.iter().map(|&t| t as u8).collect();
        assert_eq!(numbers, (1..=15).collect::<Vec<u8>>());
        assert_eq!(Tokens::BLOCK_TYPE_PROPERTY_HOOK as u8, 15);
    }

    #[test]
    fn default_is_an_empty_collection() {
        // Upstream's `new Tokens()`: no kind found, and inserting registers the new tokens.
        let mut t = Tokens::default();
        assert!(!t.is_token_kind_found(T_OPEN_TAG));
        t.insert_at(0, vec![Token::new(T_OPEN_TAG, "<?php ")]);
        assert_eq!(t.generate_code(), b"<?php ");
        assert_eq!(t.count_token_kind(T_OPEN_TAG), 1);
    }

    #[test]
    fn namespace_declarations_cached_until_changed() {
        // Like upstream's $namespaceDeclarations: computed once, recomputed after every edit that moves or changes tokens.
        let runs = std::cell::Cell::new(0);
        let count = |t: &Tokens| {
            runs.set(runs.get() + 1);
            t.count()
        };
        let mut t = Tokens::from_code(b"<?php foo(1);");
        // token_get_all('<?php foo(1);') has 6 tokens.
        assert_eq!(*t.namespace_declarations(count), 6);
        assert_eq!(*t.namespace_declarations(count), 6);
        assert_eq!(runs.get(), 1);
        // Re-setting an identical token is not a change.
        t.set(1, t[1].clone());
        t.namespace_declarations(count);
        assert_eq!(runs.get(), 1);
        type Edit = Box<dyn Fn(&mut Tokens)>;
        let mut edits: Vec<Edit> = vec![
            Box::new(|t| t.set(1, Token::new(T_STRING, "bar"))),
            Box::new(|t| t.insert_at(1, vec![Token::new(T_WHITESPACE, " ")])),
            Box::new(|t| {
                t.clear_at(1);
                // the clear above already dropped it; compute again so only clear_empty_tokens can drop it next
                t.namespace_declarations(|t| t.count());
                t.clear_empty_tokens();
            }),
            Box::new(|t| t.set_code(b"<?php $a;")),
        ];
        let expected = [6, 7, 6, 3];
        for (edit, len) in edits.drain(..).zip(expected) {
            let before = runs.get();
            edit(&mut t);
            assert_eq!(*t.namespace_declarations(count), len);
            assert_eq!(runs.get(), before + 1);
        }
    }
}
