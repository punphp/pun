//! Ports of `PhpCsFixer\Tokenizer\Transformer\*` (php-cs-fixer v3.95.17), run in the order
//! `Transformers::createSingleton()` yields on PHP 8.5 (priority desc, ties in discovery
//! order). Each runs `process($tokens, $token, $index)` for every index of the live,
//! possibly growing collection, like upstream's `foreach` over the SplFixedArray.
//!
//! Upstream throws (TypeError on a null index, UnexpectedValueException on unbalanced
//! braces) for code it cannot handle; these ports panic at the same points.

use crate::kind::Kind::{self, *};
use crate::token::{Proto, Token};
use crate::tokens::Tokens;

type Process = fn(&mut Tokens, usize);

const TRANSFORMERS: &[(&str, Process)] = &[
    ("attribute", attribute),
    ("name_qualified", name_qualified),
    ("first_class_callable", first_class_callable),
    ("class_constant", class_constant),
    ("return_ref", return_ref),
    ("namespace_operator", namespace_operator),
    ("whitespacy_comment", whitespacy_comment),
    ("brace", brace),
    ("array_typehint", array_typehint),
    ("constructor_promotion", constructor_promotion),
    ("square_brace", square_brace),
    ("import", import),
    ("brace_class_instantiation", brace_class_instantiation),
    ("use", use_),
    ("type_colon", type_colon),
    ("type_alternation", type_alternation),
    ("named_argument", named_argument),
    ("type_intersection", type_intersection),
    (
        "disjunctive_normal_form_type_parenthesis",
        dnf_type_parenthesis,
    ),
    ("nullable_type", nullable_type),
];

pub fn transform(tokens: &mut Tokens) {
    for (_, process) in TRANSFORMERS {
        let mut index = 0;
        while index < tokens.len() {
            process(tokens, index);
            index += 1;
        }
    }
}

fn idx(i: Option<usize>) -> usize {
    i.expect("Cannot access offset of type null on SplFixedArray")
}

fn prev(tokens: &Tokens, i: usize) -> usize {
    idx(tokens.get_prev_meaningful_token(i))
}

fn next(tokens: &Tokens, i: usize) -> usize {
    idx(tokens.get_next_meaningful_token(i))
}

const fn c(s: &'static str) -> Proto<'static> {
    Proto::Char(s.as_bytes())
}

const fn k(kind: Kind) -> Proto<'static> {
    Proto::Kind(kind)
}

fn set(tokens: &mut Tokens, i: usize, kind: Kind, content: &[u8]) {
    tokens.set(i, Token::new(kind, content));
}

fn attribute(tokens: &mut Tokens, mut index: usize) {
    if !tokens[index].is_kind(T_ATTRIBUTE) {
        return;
    }
    loop {
        index += 1;
        if tokens[index].equals("(", true) {
            index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index) + 1;
        }
        if tokens[index].equals("]", true) {
            break;
        }
    }
    set(tokens, index, CT_ATTRIBUTE_CLOSE, b"]");
}

/// `ImportProcessor::tokenizeName`.
pub fn tokenize_name(name: &[u8]) -> Vec<Token> {
    let mut parts: Vec<&[u8]> = name.split(|&b| b == b'\\').collect();
    let mut out = Vec::new();
    if parts[0].is_empty() {
        out.push(Token::new(T_NS_SEPARATOR, "\\"));
        parts.remove(0);
    }
    for part in parts {
        out.push(Token {
            id: Some(T_STRING),
            content: part.to_vec(),
        });
        out.push(Token::new(T_NS_SEPARATOR, "\\"));
    }
    out.pop();
    out
}

fn name_qualified(tokens: &mut Tokens, index: usize) {
    let t = &tokens[index];
    if t.is_given_kind(&[T_NAME_QUALIFIED, T_NAME_FULLY_QUALIFIED]) {
        let new = tokenize_name(&t.content);
        tokens.override_range(index, index, new);
    } else if t.is_kind(T_NAME_RELATIVE) {
        let mut new = tokenize_name(&t.content);
        new[0] = Token::new(T_NAMESPACE, "namespace");
        tokens.override_range(index, index, new);
    }
}

fn first_class_callable(tokens: &mut Tokens, index: usize) {
    if tokens[index].is_kind(T_ELLIPSIS)
        && tokens[prev(tokens, index)].equals("(", true)
        && tokens[next(tokens, index)].equals(")", true)
    {
        set(tokens, index, CT_FIRST_CLASS_CALLABLE, b"...");
    }
}

fn class_constant(tokens: &mut Tokens, index: usize) {
    if !matches!(tokens[index].id, Some(T_CLASS | T_STRING))
        || !tokens[index].equals_any(
            &[(T_CLASS, "class").into(), (T_STRING, "class").into()],
            false,
        )
    {
        return;
    }
    if tokens[prev(tokens, index)].is_kind(T_DOUBLE_COLON) {
        let content = tokens[index].content.clone();
        set(tokens, index, CT_CLASS_CONSTANT, &content);
    }
}

fn return_ref(tokens: &mut Tokens, index: usize) {
    if tokens[index].equals("&", true)
        && tokens[prev(tokens, index)].is_given_kind(&[T_FUNCTION, T_FN])
    {
        set(tokens, index, CT_RETURN_REF, b"&");
    }
}

fn namespace_operator(tokens: &mut Tokens, index: usize) {
    if !tokens[index].is_kind(T_NAMESPACE) {
        return;
    }
    if tokens[next(tokens, index)].is_kind(T_NS_SEPARATOR) {
        let content = tokens[index].content.clone();
        set(tokens, index, CT_NAMESPACE_OPERATOR, &content);
    }
}

fn whitespacy_comment(tokens: &mut Tokens, index: usize) {
    let token = &tokens[index];
    if !token.is_comment() {
        return;
    }
    let trimmed = crate::php_compat::rtrim(&token.content);
    if trimmed.len() == token.content.len() {
        return;
    }
    let whitespaces = token.content[trimmed.len()..].to_vec();
    let new = Token {
        id: token.id,
        content: trimmed.to_vec(),
    };
    tokens.set(index, new);
    if index + 1 < tokens.len() && tokens[index + 1].is_whitespace(None) {
        let merged = [&whitespaces[..], &tokens[index + 1].content[..]].concat();
        set(tokens, index + 1, T_WHITESPACE, &merged);
    } else {
        tokens.insert_at(index + 1, vec![Token::new(T_WHITESPACE, whitespaces)]);
    }
}

// --- BraceTransformer ---

fn brace(tokens: &mut Tokens, index: usize) {
    transform_into_curly_close_brace(tokens, index);
    transform_into_dollar_close_brace(tokens, index);
    transform_into_dynamic_prop_braces(tokens, index);
    transform_into_dynamic_var_braces(tokens, index);
    transform_into_property_hook_braces(tokens, index);
    // transformIntoCurlyIndexBraces: no-op on PHP >= 8.4
    transform_into_group_use_braces(tokens, index);
    transform_into_dynamic_class_constant_fetch_braces(tokens, index);
}

fn transform_into_curly_close_brace(tokens: &mut Tokens, mut index: usize) {
    if !tokens[index].is_kind(T_CURLY_OPEN) {
        return;
    }
    let mut level = 1;
    loop {
        index += 1;
        let t = &tokens[index];
        if t.equals("{", true) || t.is_kind(T_CURLY_OPEN) {
            level += 1;
        } else if t.equals("}", true) {
            level -= 1;
        }
        if level <= 0 {
            break;
        }
    }
    set(tokens, index, CT_CURLY_CLOSE, b"}");
}

fn transform_into_dollar_close_brace(tokens: &mut Tokens, index: usize) {
    if tokens[index].is_kind(T_DOLLAR_OPEN_CURLY_BRACES) {
        let n = idx(tokens.get_next_token_of_kind(index, &[c("}")], true));
        set(tokens, n, CT_DOLLAR_CLOSE_CURLY_BRACES, b"}");
    }
}

fn transform_into_dynamic_prop_braces(tokens: &mut Tokens, index: usize) {
    if !tokens[index].is_object_operator() || !tokens[index + 1].equals("{", true) {
        return;
    }
    let open = index + 1;
    let close = naively_find_curly_block_end(tokens, open);
    set(tokens, open, CT_DYNAMIC_PROP_BRACE_OPEN, b"{");
    set(tokens, close, CT_DYNAMIC_PROP_BRACE_CLOSE, b"}");
}

fn transform_into_dynamic_var_braces(tokens: &mut Tokens, index: usize) {
    if !tokens[index].equals("$", true) {
        return;
    }
    let open = match tokens.get_next_meaningful_token(index) {
        Some(open) if tokens[open].equals("{", true) => open,
        _ => return,
    };
    let close = naively_find_curly_block_end(tokens, open);
    set(tokens, open, CT_DYNAMIC_VAR_BRACE_OPEN, b"{");
    set(tokens, close, CT_DYNAMIC_VAR_BRACE_CLOSE, b"}");
}

fn transform_into_property_hook_braces(tokens: &mut Tokens, index: usize) {
    if !tokens[index].equals("{", true) {
        return;
    }
    let mut n = next(tokens, index);
    while tokens[n].is_kind(T_ATTRIBUTE) {
        n = tokens.find_block_end(Tokens::BLOCK_TYPE_ATTRIBUTE, n);
        n = next(tokens, n);
    }
    if !tokens[n].equals_any(&[(T_STRING, "get").into(), (T_STRING, "set").into()], false) {
        return;
    }
    let nn = next(tokens, n);
    if !tokens[nn].equals_any(&[c("("), c("{"), c(";"), k(T_DOUBLE_ARROW)], true) {
        return;
    }
    if tokens[nn].equals("(", true) {
        let close_paren = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, nn);
        let after = next(tokens, close_paren);
        if !tokens[after].equals_any(&[c("{"), k(T_DOUBLE_ARROW)], true) {
            return;
        }
    }
    let close = naively_find_curly_block_end(tokens, index);
    set(tokens, index, CT_PROPERTY_HOOK_BRACE_OPEN, b"{");
    set(tokens, close, CT_PROPERTY_HOOK_BRACE_CLOSE, b"}");
}

fn transform_into_group_use_braces(tokens: &mut Tokens, index: usize) {
    if !tokens[index].equals("{", true) || !tokens[prev(tokens, index)].is_kind(T_NS_SEPARATOR) {
        return;
    }
    let close = naively_find_curly_block_end(tokens, index);
    set(tokens, index, CT_GROUP_IMPORT_BRACE_OPEN, b"{");
    set(tokens, close, CT_GROUP_IMPORT_BRACE_CLOSE, b"}");
}

fn transform_into_dynamic_class_constant_fetch_braces(tokens: &mut Tokens, index: usize) {
    if !tokens[index].equals("{", true) {
        return;
    }
    let mut p = prev(tokens, index);
    while !tokens[p].is_kind(T_DOUBLE_COLON) {
        if !tokens[p].equals(")", true) {
            return;
        }
        p = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, p);
        p = prev(tokens, p);
        if !tokens[p].equals("}", true) {
            return;
        }
        p = tokens.find_block_start(Tokens::BLOCK_TYPE_BRACE, p);
        p = prev(tokens, p);
    }
    let close = naively_find_curly_block_end(tokens, index);
    let after = next(tokens, close);
    if !tokens[after].equals_any(&[c(";"), k(T_CLOSE_TAG), k(T_DOUBLE_COLON)], true) {
        return;
    }
    set(
        tokens,
        index,
        CT_DYNAMIC_CLASS_CONSTANT_FETCH_BRACE_OPEN,
        b"{",
    );
    set(
        tokens,
        close,
        CT_DYNAMIC_CLASS_CONSTANT_FETCH_BRACE_CLOSE,
        b"}",
    );
}

fn naively_find_curly_block_end(tokens: &Tokens, start: usize) -> usize {
    assert!(
        tokens[start].content == b"{",
        "Wrong start index: \"{start}\"."
    );
    let mut level = 1;
    let end = tokens.len() - 1;
    let mut index = start + 1;
    while index != end {
        let t = &tokens[index];
        if t.content == b"{" {
            level += 1;
        } else if t.content == b"}" {
            level -= 1;
            if level == 0 {
                assert!(
                    t.equals("}", true),
                    "Detected block end for index: \"{start}\" was already transformed"
                );
                return index;
            }
        }
        index += 1;
    }
    panic!("Missing block end for index: \"{start}\".");
}

fn array_typehint(tokens: &mut Tokens, index: usize) {
    if !tokens[index].is_kind(T_ARRAY) {
        return;
    }
    if !tokens[next(tokens, index)].equals("(", true) {
        let content = tokens[index].content.clone();
        set(tokens, index, CT_ARRAY_TYPEHINT, &content);
    }
}

fn constructor_promotion(tokens: &mut Tokens, index: usize) {
    if !tokens[index].is_kind(T_FUNCTION) {
        return;
    }
    let name = next(tokens, index);
    if !tokens[name].is_kind(T_STRING) || !tokens[name].content.eq_ignore_ascii_case(b"__construct")
    {
        return;
    }
    let open = next(tokens, name);
    let close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
    for i in open..close {
        let kind = match tokens[i].id {
            Some(T_PUBLIC) => CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC,
            Some(T_PROTECTED) => CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED,
            Some(T_PRIVATE) => CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE,
            _ => continue,
        };
        let content = tokens[i].content.clone();
        set(tokens, i, kind, &content);
    }
}

// --- SquareBraceTransformer ---

fn square_brace(tokens: &mut Tokens, index: usize) {
    if is_array_destructing(tokens, index) {
        transform_into_destructuring_square_brace(tokens, index);
        return;
    }
    if is_short_array(tokens, index) {
        let end = tokens.find_block_end(Tokens::BLOCK_TYPE_INDEX_BRACKET, index);
        set(tokens, index, CT_ARRAY_BRACKET_OPEN, b"[");
        set(tokens, end, CT_ARRAY_BRACKET_CLOSE, b"]");
    }
}

fn transform_into_destructuring_square_brace(tokens: &mut Tokens, index: usize) {
    let end = tokens.find_block_end(Tokens::BLOCK_TYPE_INDEX_BRACKET, index);
    set(tokens, index, CT_DESTRUCTURING_BRACKET_OPEN, b"[");
    set(tokens, end, CT_DESTRUCTURING_BRACKET_CLOSE, b"]");
    let mut previous = index;
    let mut i = next(tokens, index);
    while i < end {
        if tokens[i].equals("[", true)
            && tokens[previous].equals_any(&[k(CT_DESTRUCTURING_BRACKET_OPEN), c(",")], true)
        {
            let e = tokens.find_block_end(Tokens::BLOCK_TYPE_INDEX_BRACKET, i);
            set(tokens, e, CT_DESTRUCTURING_BRACKET_CLOSE, b"]");
            set(tokens, i, CT_DESTRUCTURING_BRACKET_OPEN, b"[");
        }
        previous = i;
        i = next(tokens, i);
    }
}

const NOT_ARRAY_PREV: &[Proto<'static>] = &[
    c(")"),
    c("]"),
    c("}"),
    c("\""),
    k(T_CONSTANT_ENCAPSED_STRING),
    k(T_STRING),
    k(T_STRING_VARNAME),
    k(T_VARIABLE),
    k(CT_ARRAY_BRACKET_CLOSE),
    k(CT_DYNAMIC_PROP_BRACE_CLOSE),
    k(CT_DYNAMIC_VAR_BRACE_CLOSE),
    k(CT_ARRAY_INDEX_BRACE_CLOSE),
];

fn is_short_array(tokens: &mut Tokens, index: usize) -> bool {
    if !tokens[index].equals("[", true) {
        return false;
    }
    if tokens[prev(tokens, index)].equals_any(NOT_ARRAY_PREV, true) {
        return false;
    }
    if tokens[next(tokens, index)].equals("]", true) {
        return true;
    }
    !is_array_destructing(tokens, index)
}

fn is_array_destructing(tokens: &mut Tokens, index: usize) -> bool {
    if !tokens[index].equals("[", true) {
        return false;
    }
    let p = prev(tokens, index);
    // Same list as is_short_array minus '}'.
    if tokens[p].equals_any(&NOT_ARRAY_PREV[..2], true)
        || tokens[p].equals_any(&NOT_ARRAY_PREV[3..], true)
    {
        return false;
    }
    if tokens[p].is_kind(T_AS) {
        return true;
    }
    if tokens[p].is_kind(T_DOUBLE_ARROW) {
        let var = prev(tokens, p);
        if !tokens[var].is_kind(T_VARIABLE) {
            return false;
        }
        if tokens[prev(tokens, var)].is_kind(T_AS) {
            return true;
        }
    }
    let (t, _) = Tokens::detect_block_type(&tokens[index]).expect("block type");
    let end = tokens.find_block_end(t, index);
    tokens[next(tokens, end)].equals("=", true)
}

fn import(tokens: &mut Tokens, index: usize) {
    let Some(id @ (T_CONST | T_FUNCTION)) = tokens[index].id else {
        return;
    };
    if !tokens[prev(tokens, index)].is_kind(T_USE) {
        let n = idx(tokens.get_next_token_of_kind(
            index,
            &[
                c("="),
                c("("),
                k(CT_RETURN_REF),
                k(CT_GROUP_IMPORT_BRACE_CLOSE),
            ],
            true,
        ));
        if !tokens[n].is_kind(CT_GROUP_IMPORT_BRACE_CLOSE) {
            return;
        }
    }
    let content = tokens[index].content.clone();
    set(
        tokens,
        index,
        if id == T_FUNCTION {
            CT_FUNCTION_IMPORT
        } else {
            CT_CONST_IMPORT
        },
        &content,
    );
}

fn brace_class_instantiation(tokens: &mut Tokens, index: usize) {
    if !tokens[index].equals("(", true) || !tokens[next(tokens, index)].is_kind(T_NEW) {
        return;
    }
    if tokens[prev(tokens, index)].equals_any(
        &[
            c(")"),
            c("]"),
            k(CT_ARRAY_INDEX_BRACE_CLOSE),
            k(CT_ARRAY_BRACKET_CLOSE),
            k(CT_CLASS_INSTANTIATION_PARENTHESIS_CLOSE),
            k(T_ARRAY),
            k(T_CLASS),
            k(T_ELSEIF),
            k(T_FOR),
            k(T_FOREACH),
            k(T_IF),
            k(T_STATIC),
            k(T_STRING),
            k(T_SWITCH),
            k(T_VARIABLE),
            k(T_WHILE),
        ],
        true,
    ) {
        return;
    }
    let close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index);
    set(tokens, index, CT_CLASS_INSTANTIATION_PARENTHESIS_OPEN, b"(");
    set(
        tokens,
        close,
        CT_CLASS_INSTANTIATION_PARENTHESIS_CLOSE,
        b")",
    );
}

fn use_(tokens: &mut Tokens, index: usize) {
    let is_use_for_lambda = |tokens: &Tokens, i: usize| tokens[next(tokens, i)].equals("(", true);
    if tokens[index].is_kind(T_USE) && is_use_for_lambda(tokens, index) {
        let content = tokens[index].content.clone();
        set(tokens, index, CT_USE_LAMBDA, &content);
        return;
    }
    // Upstream also skips `::class` here, but class_constant has already made that CT_CLASS_CONSTANT.
    if !tokens[index].is_given_kind(&[T_CLASS, T_TRAIT, T_ENUM]) {
        return;
    }
    let mut index = idx(tokens.get_next_token_of_kind(index, &[c("{")], true));
    let inner_limit = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index);
    while index < inner_limit {
        index += 1;
        if !tokens[index].is_kind(T_USE) {
            continue;
        }
        let kind = if is_use_for_lambda(tokens, index) {
            CT_USE_LAMBDA
        } else {
            CT_USE_TRAIT
        };
        let content = tokens[index].content.clone();
        set(tokens, index, kind, &content);
    }
}

fn type_colon(tokens: &mut Tokens, index: usize) {
    if !tokens[index].equals(":", true) {
        return;
    }
    let end = prev(tokens, index);
    if tokens[prev(tokens, end)].is_kind(T_ENUM) {
        set(tokens, index, CT_TYPE_COLON, b":");
        return;
    }
    if !tokens[end].equals(")", true) {
        return;
    }
    let start = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, end);
    let mut p = prev(tokens, start);
    if tokens[p].is_kind(T_STRING) {
        p = prev(tokens, p);
    }
    if tokens[p].is_given_kind(&[T_FUNCTION, CT_RETURN_REF, CT_USE_LAMBDA, T_FN]) {
        set(tokens, index, CT_TYPE_COLON, b":");
    }
}

// --- AbstractTypeTransformer ---

const TYPE_END_TOKENS: &[Proto<'static>] = &[
    c(")"),
    k(T_CALLABLE),
    k(T_NS_SEPARATOR),
    k(T_STATIC),
    k(T_STRING),
    k(CT_ARRAY_TYPEHINT),
];

const TYPE_TOKENS: &[Proto<'static>] = &[
    c("|"),
    c("&"),
    c("("),
    c(")"),
    k(T_CALLABLE),
    k(T_NS_SEPARATOR),
    k(T_STATIC),
    k(T_STRING),
    k(CT_ARRAY_TYPEHINT),
    k(CT_TYPE_ALTERNATION),
    k(CT_TYPE_INTERSECTION),
    k(T_WHITESPACE),
    k(T_COMMENT),
    k(T_DOC_COMMENT),
];

fn do_process(
    tokens: &mut Tokens,
    index: usize,
    original: Proto<'_>,
    replacement: Kind,
    content: &[u8],
) {
    if !tokens[index].equals(original, true) || !is_part_of_type(tokens, index) {
        return;
    }
    set(tokens, index, replacement, content);
}

fn is_part_of_type(tokens: &Tokens, index: usize) -> bool {
    let colon = idx(tokens.get_token_not_of_kind_sibling(index, -1, TYPE_TOKENS));
    if tokens[colon].is_given_kind(&[T_CATCH, CT_TYPE_COLON, T_CONST]) {
        return true;
    }
    let after = idx(tokens.get_token_not_of_kind_sibling(index, 1, TYPE_TOKENS));
    if tokens[after].is_kind(T_ELLIPSIS) {
        return true;
    }
    if !tokens[after].is_kind(T_VARIABLE) {
        return false;
    }
    let before_var = prev(tokens, after);
    if tokens[before_var].equals("&", true) {
        let p = tokens.get_prev_token_of_kind(
            index,
            &[
                c("{"),
                c("}"),
                c(";"),
                k(T_CLOSE_TAG),
                k(T_FN),
                k(T_FUNCTION),
            ],
            true,
        );
        return p.is_some_and(|p| tokens[p].is_given_kind(&[T_FN, T_FUNCTION]));
    }
    tokens[before_var].equals_any(TYPE_END_TOKENS, true)
}

fn type_alternation(tokens: &mut Tokens, index: usize) {
    do_process(tokens, index, c("|"), CT_TYPE_ALTERNATION, b"|");
}

fn type_intersection(tokens: &mut Tokens, index: usize) {
    do_process(
        tokens,
        index,
        Proto::KindContent(T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG, b"&"),
        CT_TYPE_INTERSECTION,
        b"&",
    );
}

fn named_argument(tokens: &mut Tokens, index: usize) {
    if !tokens[index].equals(":", true) {
        return;
    }
    let s = prev(tokens, index);
    if !tokens[s].is_kind(T_STRING) {
        return;
    }
    let pre = prev(tokens, s);
    if !tokens[pre].equals_any(&[c(","), c("(")], true) {
        return;
    }
    let content = tokens[s].content.clone();
    set(tokens, s, CT_NAMED_ARGUMENT_NAME, &content);
    set(tokens, index, CT_NAMED_ARGUMENT_COLON, b":");
}

fn dnf_type_parenthesis(tokens: &mut Tokens, index: usize) {
    let (open, close);
    if tokens[index].equals("(", true) && tokens[prev(tokens, index)].is_kind(CT_TYPE_ALTERNATION) {
        open = index;
        close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index);
    } else if tokens[index].equals(")", true)
        && tokens[next(tokens, index)].is_kind(CT_TYPE_ALTERNATION)
    {
        open = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, index);
        close = index;
    } else {
        return;
    }
    set(
        tokens,
        open,
        CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_OPEN,
        b"(",
    );
    set(
        tokens,
        close,
        CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_CLOSE,
        b")",
    );
}

fn nullable_type(tokens: &mut Tokens, index: usize) {
    if !tokens[index].equals("?", true) {
        return;
    }
    let p = prev(tokens, index);
    if !tokens[p].equals_any(
        &[
            c("("),
            c(","),
            k(CT_TYPE_COLON),
            k(CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC),
            k(CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED),
            k(CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE),
            k(CT_ATTRIBUTE_CLOSE),
            k(T_PRIVATE),
            k(T_PROTECTED),
            k(T_PUBLIC),
            k(T_VAR),
            k(T_STATIC),
            k(T_CONST),
            k(T_ABSTRACT),
            k(T_FINAL),
            k(T_READONLY),
            k(T_PRIVATE_SET),
            k(T_PROTECTED_SET),
            k(T_PUBLIC_SET),
        ],
        true,
    ) {
        return;
    }
    if tokens[p].is_kind(T_STATIC) && tokens[prev(tokens, p)].is_kind(T_INSTANCEOF) {
        return;
    }
    set(tokens, index, CT_NULLABLE_TYPE, b"?");
}
