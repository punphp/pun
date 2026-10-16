//! Port of `PhpCsFixer\Fixer\LanguageConstruct\SingleSpaceAroundConstructFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{php_compat, Kind, Kind::*, Token, Tokens};
use serde_json::Value;
use std::path::Path;

const TOKEN_MAP_CONTAIN_A_SINGLE_SPACE: &[(&str, Kind)] = &[("yield_from", T_YIELD_FROM)];

const TOKEN_MAP_PRECEDED_BY_A_SINGLE_SPACE: &[(&str, Kind)] =
    &[("as", T_AS), ("else", T_ELSE), ("elseif", T_ELSEIF), ("use_lambda", CT_USE_LAMBDA)];

const TOKEN_MAP_FOLLOWED_BY_A_SINGLE_SPACE: &[(&str, Kind)] = &[
    ("abstract", T_ABSTRACT), ("as", T_AS), ("attribute", CT_ATTRIBUTE_CLOSE), ("break", T_BREAK), ("case", T_CASE),
    ("catch", T_CATCH), ("class", T_CLASS), ("clone", T_CLONE), ("comment", T_COMMENT), ("const", T_CONST),
    ("const_import", CT_CONST_IMPORT), ("continue", T_CONTINUE), ("do", T_DO), ("echo", T_ECHO), ("else", T_ELSE),
    ("elseif", T_ELSEIF), ("enum", T_ENUM), ("extends", T_EXTENDS), ("final", T_FINAL), ("finally", T_FINALLY),
    ("for", T_FOR), ("foreach", T_FOREACH), ("function", T_FUNCTION), ("function_import", CT_FUNCTION_IMPORT),
    ("global", T_GLOBAL), ("goto", T_GOTO), ("if", T_IF), ("implements", T_IMPLEMENTS), ("include", T_INCLUDE),
    ("include_once", T_INCLUDE_ONCE), ("instanceof", T_INSTANCEOF), ("insteadof", T_INSTEADOF),
    ("interface", T_INTERFACE), ("match", T_MATCH), ("named_argument", CT_NAMED_ARGUMENT_COLON),
    ("namespace", T_NAMESPACE), ("new", T_NEW), ("open_tag_with_echo", T_OPEN_TAG_WITH_ECHO), ("php_doc", T_DOC_COMMENT),
    ("php_open", T_OPEN_TAG), ("print", T_PRINT), ("private", T_PRIVATE), ("private_set", T_PRIVATE_SET),
    ("protected", T_PROTECTED), ("protected_set", T_PROTECTED_SET), ("public", T_PUBLIC), ("public_set", T_PUBLIC_SET),
    ("readonly", T_READONLY), ("require", T_REQUIRE), ("require_once", T_REQUIRE_ONCE), ("return", T_RETURN),
    ("static", T_STATIC), ("switch", T_SWITCH), ("throw", T_THROW), ("trait", T_TRAIT), ("try", T_TRY),
    ("type_colon", CT_TYPE_COLON), ("use", T_USE), ("use_lambda", CT_USE_LAMBDA), ("use_trait", CT_USE_TRAIT),
    ("var", T_VAR), ("while", T_WHILE), ("yield", T_YIELD), ("yield_from", T_YIELD_FROM),
];

struct SingleSpaceAroundConstructFixer {
    contain: Vec<Kind>,
    preceded: Vec<Kind>,
    followed: Vec<Kind>,
}

fn keys(map: &[(&'static str, Kind)]) -> Vec<&'static str> {
    map.iter().map(|(k, _)| *k).collect()
}

fn kinds(map: &[(&str, Kind)], selected: &[String]) -> Vec<Kind> {
    selected.iter().map(|s| map.iter().find(|(k, _)| k == s).unwrap().1).collect()
}

pub fn fixer() -> Box<dyn Fixer> {
    let mut f = SingleSpaceAroundConstructFixer { contain: vec![], preceded: vec![], followed: vec![] };
    f.configure(&Value::Null).unwrap();
    Box::new(f)
}

impl Fixer for SingleSpaceAroundConstructFixer {
    fn name(&self) -> &'static str {
        "single_space_around_construct"
    }
    fn priority(&self) -> i32 {
        36
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&self.contain)
            || tokens.is_any_token_kinds_found(&self.preceded)
            || tokens.is_any_token_kinds_found(&self.followed)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(
            self.name(),
            options,
            &["constructs_contain_a_single_space", "constructs_preceded_by_a_single_space", "constructs_followed_by_a_single_space"],
        )?;
        let contain_keys = keys(TOKEN_MAP_CONTAIN_A_SINGLE_SPACE);
        let preceded_keys = keys(TOKEN_MAP_PRECEDED_BY_A_SINGLE_SPACE);
        let followed_keys = keys(TOKEN_MAP_FOLLOWED_BY_A_SINGLE_SPACE);
        let contain = o.strings("constructs_contain_a_single_space", &contain_keys, &contain_keys)?;
        let preceded = o.strings("constructs_preceded_by_a_single_space", &["as", "use_lambda"], &preceded_keys)?;
        let followed = o.strings("constructs_followed_by_a_single_space", &followed_keys, &followed_keys)?;
        self.contain = kinds(TOKEN_MAP_CONTAIN_A_SINGLE_SPACE, &contain);
        self.preceded = kinds(TOKEN_MAP_PRECEDED_BY_A_SINGLE_SPACE, &preceded);
        self.followed = kinds(TOKEN_MAP_FOLLOWED_BY_A_SINGLE_SPACE, &followed);
        let has = |k: &str| followed.iter().any(|f| f == k);
        if has("public") {
            self.followed.push(CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC);
        }
        if has("protected") {
            self.followed.push(CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED);
        }
        if has("private") {
            self.followed.push(CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE);
        }
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (1..tokens.count()).rev() {
            if tokens[index].is_given_kind(&self.contain) {
                let token = &tokens[index];
                if token.is_kind(T_YIELD_FROM) && php_compat::strtolower(&token.content) != b"yield from" {
                    let content = Preg::replace("/\\s+/", " ", &token.content);
                    tokens.set(index, Token::new(T_YIELD_FROM, content));
                }
            }
        }

        for index in (1..tokens.count()).rev() {
            if tokens[index].is_given_kind(&self.preceded) && !is_full_line_comment_before(tokens, index) {
                tokens.ensure_whitespace_at_index(index - 1, 1, b" ");
            }
        }

        if tokens.count() < 2 {
            return;
        }
        for index in (0..=tokens.count() - 2).rev() {
            let token = &tokens[index];
            if !token.is_given_kind(&self.followed) {
                continue;
            }
            let whitespace_token_index = index + 1;
            if tokens[whitespace_token_index].equals_any(
                &[",".into(), ":".into(), ";".into(), ")".into(), CT_ARRAY_BRACKET_CLOSE.into(), CT_DESTRUCTURING_BRACKET_CLOSE.into()],
                true,
            ) {
                continue;
            }
            if token.is_kind(T_STATIC)
                && !tokens[tokens.get_next_meaningful_token(index).unwrap()].is_given_kind(&[
                    T_FN, T_FUNCTION, T_NS_SEPARATOR, T_STRING, T_VARIABLE, CT_ARRAY_TYPEHINT, CT_NULLABLE_TYPE,
                ])
            {
                continue;
            }
            if token.is_kind(T_OPEN_TAG) {
                let ws = &tokens[whitespace_token_index];
                if ws.is_kind(T_WHITESPACE) && !ws.content.contains(&b'\n') && !token.content.contains(&b'\n') {
                    tokens.clear_at(whitespace_token_index);
                }
                continue;
            }
            if token.is_kind(T_CLASS) && tokens[tokens.get_next_meaningful_token(index).unwrap()].equals("(", true) {
                continue;
            }
            if token.is_given_kind(&[T_EXTENDS, T_IMPLEMENTS]) && is_multiline_extends_or_implements_with_more_than_one_ancestor(tokens, index) {
                continue;
            }
            if token.is_kind(T_RETURN) && is_multi_line_return(tokens, index) {
                continue;
            }
            if token.is_kind(T_CONST) && is_multiline_comma_separated_constant(tokens, index) {
                continue;
            }
            if token.is_comment() || token.is_kind(CT_ATTRIBUTE_CLOSE) {
                let ws = &tokens[whitespace_token_index];
                if ws.is_kind(T_WHITESPACE) && ws.content.contains(&b'\n') {
                    continue;
                }
            }
            if tokens[whitespace_token_index].is_whitespace(None) && tokens[whitespace_token_index].content.contains(&b'\n') {
                let next_next_token = &tokens[whitespace_token_index + 1];
                if next_next_token.is_kind(T_ATTRIBUTE) || (next_next_token.is_comment() && next_next_token.content.starts_with(b"#[")) {
                    continue;
                }
                if next_next_token.is_kind(T_DOC_COMMENT) {
                    continue;
                }
            }
            tokens.ensure_whitespace_at_index(whitespace_token_index, 0, b" ");
        }
    }
}

fn is_multi_line_return(tokens: &Tokens, mut index: usize) -> bool {
    index += 1;
    let token_following_return = &tokens[index];
    if !token_following_return.is_kind(T_WHITESPACE) || !token_following_return.content.contains(&b'\n') {
        return false;
    }
    let mut nested_count = 0;
    let index_end = tokens.count() - 1;
    index += 1;
    while index < index_end {
        if tokens[index].content.contains(&b'\n') {
            return true;
        }
        if tokens[index].equals("{", true) {
            nested_count += 1;
        } else if tokens[index].equals("}", true) {
            nested_count -= 1;
        } else if nested_count == 0 && tokens[index].equals_any(&[";".into(), T_CLOSE_TAG.into()], true) {
            break;
        }
        index += 1;
    }
    false
}

fn is_multiline_extends_or_implements_with_more_than_one_ancestor(tokens: &Tokens, mut index: usize) -> bool {
    let mut has_more_than_one_ancestor = false;
    loop {
        index += 1;
        let token = &tokens[index];
        if token.equals(",", true) {
            has_more_than_one_ancestor = true;
            continue;
        }
        if token.equals("{", true) {
            return false;
        }
        if has_more_than_one_ancestor && token.content.contains(&b'\n') {
            return true;
        }
    }
}

fn is_multiline_comma_separated_constant(tokens: &Tokens, constant_index: usize) -> bool {
    let mut is_multiline_constant = false;
    let mut has_more_than_one_constant = false;
    let mut index = constant_index;
    while !tokens[index].equals_any(&[";".into(), T_CLOSE_TAG.into()], true) {
        index += 1;
        is_multiline_constant = is_multiline_constant || tokens[index].content.contains(&b'\n');
        if tokens[index].equals(",", true) {
            has_more_than_one_constant = true;
        }
        if let Some((t, true)) = Tokens::detect_block_type(&tokens[index]) {
            index = tokens.find_block_end(t, index);
        }
    }
    has_more_than_one_constant && is_multiline_constant
}

fn is_full_line_comment_before(tokens: &Tokens, index: usize) -> bool {
    let before_index = tokens.get_prev_non_whitespace(index, None).unwrap();
    if !tokens[before_index].is_kind(T_COMMENT) {
        return false;
    }
    let content = &tokens[before_index].content;
    content.starts_with(b"#") || content.starts_with(b"//")
}
