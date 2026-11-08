//! Port of `PhpCsFixer\Fixer\FunctionNotation\DateTimeCreateFromFormatCallFixer`.
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::namespace_uses_analyzer::NamespaceUsesAnalyzer;
use pun_format::Fixer;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Kind::*, Token, Tokens};
use std::collections::BTreeMap;
use std::path::Path;

struct DateTimeCreateFromFormatCallFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(DateTimeCreateFromFormatCallFixer)
}

impl Fixer for DateTimeCreateFromFormatCallFixer {
    fn name(&self) -> &'static str {
        "date_time_create_from_format_call"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOUBLE_COLON)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let namespaces = tokens.namespace_declarations();
        for namespace in namespaces.iter() {
            let scope_start_index = namespace.scope_start_index;
            let use_declarations =
                NamespaceUsesAnalyzer.get_declarations_in_namespace(tokens, namespace, false);
            let mut index = namespace.scope_end_index;
            'tokens: while index > scope_start_index {
                let i = index;
                index -= 1;
                if !tokens[i].is_kind(T_DOUBLE_COLON) {
                    continue;
                }
                let function_name_index = tokens.get_next_meaningful_token(i).unwrap();
                if !tokens[function_name_index].equals((T_STRING, "createFromFormat"), false) {
                    continue;
                }
                let next = tokens
                    .get_next_meaningful_token(function_name_index)
                    .unwrap();
                if !tokens[next].equals("(", true) {
                    continue;
                }
                let class_name_index = tokens.get_prev_meaningful_token(i).unwrap();
                if !tokens[class_name_index].equals_any(
                    &[
                        (T_STRING, "DateTime").into(),
                        (T_STRING, "DateTimeImmutable").into(),
                    ],
                    false,
                ) {
                    continue;
                }
                let pre_class_name_index =
                    tokens.get_prev_meaningful_token(class_name_index).unwrap();
                if tokens[pre_class_name_index].is_kind(T_NS_SEPARATOR) {
                    let before = tokens
                        .get_prev_meaningful_token(pre_class_name_index)
                        .unwrap();
                    if tokens[before].is_kind(T_STRING) {
                        continue;
                    }
                } else if !namespace.is_global_namespace() {
                    continue;
                } else {
                    for use_declaration in &use_declarations {
                        for name in [&b"datetime"[..], b"datetimeimmutable"] {
                            if name == strtolower(&use_declaration.short_name)
                                && name != strtolower(&use_declaration.full_name)
                            {
                                continue 'tokens;
                            }
                        }
                    }
                }
                let open_index = tokens
                    .get_next_token_of_kind(function_name_index, &["(".into()], true)
                    .unwrap();
                let close_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_index);
                let arguments = ArgumentsAnalyzer.get_arguments(tokens, open_index, close_index);
                let Some(argument_index) = get_first_argument_token_index(tokens, &arguments)
                else {
                    continue;
                };
                let format = tokens[argument_index].get_content();
                if format.len() < 3 {
                    continue;
                }
                let offset = if format[0] == b'b' || format[0] == b'B' {
                    2
                } else {
                    1
                };
                if format[offset] == b'!' {
                    continue;
                }
                let mut new_format = format.to_vec();
                new_format.insert(offset, b'!');
                tokens.clear_at(argument_index);
                tokens.insert_at(
                    argument_index,
                    vec![Token::new(T_CONSTANT_ENCAPSED_STRING, new_format)],
                );
            }
        }
    }
}

fn get_first_argument_token_index(
    tokens: &Tokens,
    arguments: &BTreeMap<usize, usize>,
) -> Option<usize> {
    if arguments.len() != 2 {
        return None;
    }
    let (&start, &argument_end_index) = arguments.iter().next().unwrap();
    let argument_start_index = tokens.get_next_meaningful_token(start - 1).unwrap();
    if argument_start_index != argument_end_index
        && tokens
            .get_next_meaningful_token(argument_start_index)
            .is_some_and(|n| n <= argument_end_index)
    {
        return None; // argument is not a simple single string
    }
    tokens[argument_start_index]
        .is_kind(T_CONSTANT_ENCAPSED_STRING)
        .then_some(argument_start_index)
}
