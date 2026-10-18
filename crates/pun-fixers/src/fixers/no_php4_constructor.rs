//! Port of `PhpCsFixer\Fixer\ClassNotation\NoPhp4ConstructorFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::{self, *};
use pun_tokens::token::OBJECT_OPERATOR_KINDS;
use pun_tokens::{Proto, Token, Tokens, TokensAnalyzer};
use std::path::Path;

struct NoPhp4ConstructorFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoPhp4ConstructorFixer)
}

/// `findFunction`'s array.
struct Function {
    name_index: usize,
    start_index: usize,
    end_index: Option<usize>,
    body_index: Option<usize>,
    is_abstract: bool,
    is_static: bool,
}

impl Fixer for NoPhp4ConstructorFixer {
    fn name(&self) -> &'static str {
        "no_php4_constructor"
    }
    fn priority(&self) -> i32 {
        75
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_CLASS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let classes = tokens.find_given_kind(T_CLASS, 0, None);
        let num_classes = classes.len();
        let mut i = 0;
        while i < num_classes {
            let index = classes[i];
            if TokensAnalyzer::new(tokens).is_anonymous_class(index) {
                i += 1;
                continue;
            }
            let nsp_index =
                tokens.get_prev_token_of_kind(index, &[(T_NAMESPACE, "namespace").into()], true);
            if let Some(nsp_index) = nsp_index {
                let mut nsp_index = tokens.get_next_meaningful_token(nsp_index).unwrap();
                if !tokens[nsp_index].equals("{", true) {
                    nsp_index = tokens
                        .get_next_token_of_kind(nsp_index, &[";".into(), "{".into()], true)
                        .unwrap();
                    if tokens[nsp_index].equals(";", true) {
                        break;
                    }
                    let nsp_end = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, nsp_index);
                    if index < nsp_end {
                        for &class in &classes[i + 1..] {
                            if class < nsp_end {
                                i += 1;
                            }
                        }
                        i += 1;
                        continue;
                    }
                }
            }
            let class_name_index = tokens.get_next_meaningful_token(index).unwrap();
            let class_name = tokens[class_name_index].get_content().to_vec();
            let class_start = tokens
                .get_next_token_of_kind(class_name_index, &["{".into()], true)
                .unwrap();
            let class_end = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, class_start);
            fix_constructor(tokens, &class_name, class_start, class_end);
            fix_parent(tokens, class_start, class_end);
            i += 1;
        }
    }
}

fn fix_constructor(tokens: &mut Tokens, class_name: &[u8], class_start: usize, class_end: usize) {
    let Some(php4) = find_function(tokens, class_name, class_start, class_end) else {
        return; // no PHP4-constructor!
    };
    if php4.is_abstract || php4.is_static {
        return; // PHP4 constructor can't be abstract or static
    }
    let body4 = php4.body_index.unwrap();
    let Some(php5) = find_function(tokens, b"__construct", class_start, class_end) else {
        tokens.set(php4.name_index, Token::new(T_STRING, "__construct"));
        fix_infinite_recursion(tokens, body4, php4.end_index.unwrap());
        return;
    };
    for seq in get_wrapper_method_sequence(tokens, b"__construct", php4.start_index, body4) {
        let seq = protos(&seq);
        if tokens
            .find_sequence(&seq, body4 - 1, php4.end_index, |k| k != 3)
            .is_some()
        {
            for i in php4.start_index..=php4.end_index.unwrap() {
                tokens.clear_at(i);
            }
            return;
        }
    }
    for seq in get_wrapper_method_sequence(tokens, class_name, php4.start_index, body4) {
        let seq = protos(&seq);
        if tokens
            .find_sequence(&seq, php5.body_index.unwrap() - 1, php5.end_index, |k| {
                k != 3
            })
            .is_some()
        {
            for i in php5.start_index..=php5.end_index.unwrap() {
                tokens.clear_at(i);
            }
            tokens.set(php4.name_index, Token::new(T_STRING, "__construct"));
            return;
        }
    }
}

fn fix_parent(tokens: &mut Tokens, class_start: usize, class_end: usize) {
    for index in tokens.find_given_kind(T_EXTENDS, 0, None) {
        let parent_index = tokens.get_next_meaningful_token(index).unwrap();
        let parent_class = tokens[parent_index].get_content().to_vec();
        let parent_seq = tokens.find_sequence(
            &[
                T_STRING.into(),
                T_DOUBLE_COLON.into(),
                Proto::KindContent(T_STRING, &parent_class),
                "(".into(),
            ],
            class_start,
            Some(class_end),
            |k| k != 2,
        );
        if let Some(parent_seq) = parent_seq
            && tokens[parent_seq[0]].equals_any(
                &[
                    (T_STRING, "parent").into(),
                    Proto::KindContent(T_STRING, &parent_class),
                ],
                false,
            )
        {
            tokens.set(parent_seq[0], Token::new(T_STRING, "parent"));
            tokens.set(parent_seq[2], Token::new(T_STRING, "__construct"));
        }
        for &object_operator_kind in OBJECT_OPERATOR_KINDS {
            let parent_seq = tokens.find_sequence(
                &[
                    (T_VARIABLE, "$this").into(),
                    object_operator_kind.into(),
                    Proto::KindContent(T_STRING, &parent_class),
                    "(".into(),
                ],
                class_start,
                Some(class_end),
                |k| k != 2,
            );
            if let Some(parent_seq) = parent_seq {
                tokens.set(parent_seq[0], Token::new(T_STRING, "parent"));
                tokens.set(parent_seq[1], Token::new(T_DOUBLE_COLON, "::"));
                tokens.set(parent_seq[2], Token::new(T_STRING, "__construct"));
            }
        }
    }
}

fn fix_infinite_recursion(tokens: &mut Tokens, start: usize, end: usize) {
    for &object_operator_kind in OBJECT_OPERATOR_KINDS {
        let seq = [
            (T_VARIABLE, "$this").into(),
            object_operator_kind.into(),
            (T_STRING, "__construct").into(),
        ];
        loop {
            let Some(call_seq) = tokens.find_sequence(&seq, start, Some(end), |k| k != 2) else {
                return;
            };
            tokens.set(call_seq[0], Token::new(T_STRING, "parent"));
            tokens.set(call_seq[1], Token::new(T_DOUBLE_COLON, "::"));
        }
    }
}

/// An owned sequence element: a char, or a kind with content.
enum Item {
    Char(&'static str),
    Kind(Kind),
    KindContent(Kind, Vec<u8>),
}

fn protos(seq: &[Item]) -> Vec<Proto<'_>> {
    seq.iter()
        .map(|item| match item {
            Item::Char(c) => (*c).into(),
            Item::Kind(k) => (*k).into(),
            Item::KindContent(k, c) => Proto::KindContent(*k, c),
        })
        .collect()
}

/// The sequences of a method body that only calls `$this->method(...)` with the parameters in order; the method
/// name compares case-insensitively.
fn get_wrapper_method_sequence(
    tokens: &Tokens,
    method: &[u8],
    start_index: usize,
    body_index: usize,
) -> Vec<Vec<Item>> {
    let mut sequences = vec![];
    for &object_operator_kind in OBJECT_OPERATOR_KINDS {
        let mut seq = vec![
            Item::Char("{"),
            Item::KindContent(T_VARIABLE, b"$this".to_vec()),
            Item::Kind(object_operator_kind),
            Item::KindContent(T_STRING, method.to_vec()),
            Item::Char("("),
        ];
        let mut index = start_index;
        while let Some(i) = tokens.get_next_token_of_kind(index, &[T_VARIABLE.into()], true)
            && i < body_index
        {
            index = i;
            if seq.len() > 5 {
                seq.push(Item::Char(","));
            }
            seq.push(Item::KindContent(
                T_VARIABLE,
                tokens[index].get_content().to_vec(),
            ));
        }
        seq.push(Item::Char(")"));
        seq.push(Item::Char(";"));
        seq.push(Item::Char("}"));
        sequences.push(seq);
    }
    sequences
}

fn find_function(
    tokens: &Tokens,
    name: &[u8],
    start_index: usize,
    end_index: usize,
) -> Option<Function> {
    let function = tokens.find_sequence(
        &[
            T_FUNCTION.into(),
            Proto::KindContent(T_STRING, name),
            "(".into(),
        ],
        start_index,
        Some(end_index),
        |_| false,
    )?;
    let possible_modifiers = [
        T_PUBLIC,
        T_PROTECTED,
        T_PRIVATE,
        T_STATIC,
        T_ABSTRACT,
        T_FINAL,
    ];
    let (mut is_abstract, mut is_static) = (false, false);
    let mut prev_block = tokens.get_prev_meaningful_token(function[0]).unwrap();
    while tokens[prev_block].is_given_kind(&possible_modifiers) {
        is_abstract |= tokens[prev_block].is_kind(T_ABSTRACT);
        is_static |= tokens[prev_block].is_kind(T_STATIC);
        prev_block = tokens.get_prev_meaningful_token(prev_block).unwrap();
    }
    let (body_start, func_end) = if is_abstract {
        (
            None,
            tokens.get_next_token_of_kind(function[2], &[";".into()], true),
        )
    } else {
        let body_start = tokens.get_next_token_of_kind(function[2], &["{".into()], true);
        (
            body_start,
            body_start.map(|b| tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, b)),
        )
    };
    Some(Function {
        name_index: function[1],
        start_index: prev_block + 1,
        end_index: func_end,
        body_index: body_start,
        is_abstract,
        is_static,
    })
}
