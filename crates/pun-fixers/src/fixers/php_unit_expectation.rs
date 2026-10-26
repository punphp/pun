//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitExpectationFixer`.
use crate::analyzers::abstract_php_unit_fixer::{apply_fix, is_candidate, target_fulfills};
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::whitespaces_analyzer::WhitespacesAnalyzer;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::{self, *};
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct PhpUnitExpectationFixer {
    method_map: Vec<(&'static str, &'static str)>,
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitExpectationFixer {
        method_map: method_map("newest"),
        whitespaces_config: WhitespacesFixerConfig::default(),
    })
}

/// `configurePostNormalisation`.
fn method_map(target: &str) -> Vec<(&'static str, &'static str)> {
    let mut map = vec![("setExpectedException", "expectExceptionMessage")];
    if target_fulfills(target, "8.4") {
        map.push((
            "setExpectedExceptionRegExp",
            "expectExceptionMessageMatches",
        ));
        map.push((
            "expectExceptionMessageRegExp",
            "expectExceptionMessageMatches",
        ));
    } else if target_fulfills(target, "5.6") {
        map.push(("setExpectedExceptionRegExp", "expectExceptionMessageRegExp"));
    }
    map
}

impl Fixer for PhpUnitExpectationFixer {
    fn name(&self) -> &'static str {
        "php_unit_expectation"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["target"])?;
        let target = c.string("target", "newest", &["5.2", "5.6", "8.4", "newest"])?;
        self.method_map = method_map(&target);
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, |tokens, start_index, _| {
            for object_operator in [T_OBJECT_OPERATOR, T_NULLSAFE_OBJECT_OPERATOR] {
                self.apply_with_object_operator(tokens, start_index, object_operator);
            }
        });
    }
}

impl PhpUnitExpectationFixer {
    /// `applyPhpUnitClassFixWithObjectOperator`: upstream's loop ends only when no `$this->name` follows, so the end
    /// of the class does not bound it.
    fn apply_with_object_operator(
        &self,
        tokens: &mut Tokens,
        start_index: usize,
        object_operator: Kind,
    ) {
        let old_method_sequence = [
            (T_VARIABLE, "$this").into(),
            object_operator.into(),
            T_STRING.into(),
        ];
        let mut index = start_index;
        loop {
            let Some(found) = tokens.find_sequence(&old_method_sequence, index, None, |_| true)
            else {
                return;
            };
            let this_index = found[0];
            index = found[2];
            let content = tokens[index].get_content();
            let Some(&(_, new_expected_exception_name)) = self
                .method_map
                .iter()
                .find(|(from, _)| from.as_bytes() == content)
            else {
                index += 1;
                continue;
            };
            let open_index = tokens
                .get_next_token_of_kind(index, &["(".into()], true)
                .unwrap();
            let close_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_index);
            let comma_index = tokens.get_prev_meaningful_token(close_index).unwrap();
            if tokens[comma_index].equals(",", true) {
                tokens.remove_trailing_whitespace(comma_index, None);
                tokens.clear_at(comma_index);
            }
            let arguments: Vec<usize> = ArgumentsAnalyzer
                .get_arguments(tokens, open_index, close_index)
                .into_keys()
                .collect();
            let arguments_replacements = [
                "expectException",
                new_expected_exception_name,
                "expectExceptionCode",
            ];
            let mut indent = self
                .whitespaces_config
                .get_line_ending()
                .as_bytes()
                .to_vec();
            indent.extend(WhitespacesAnalyzer::detect_indent(tokens, this_index));
            let mut is_multiline_whitespace = false;
            for cnt in (1..arguments.len()).rev() {
                let arg_start = arguments[cnt];
                let arg_before = tokens.get_prev_meaningful_token(arg_start).unwrap();
                // Upstream throws `Unexpected index %d to find replacement method.` for a fourth argument.
                let replacement = arguments_replacements[cnt];
                if replacement == "expectExceptionMessage" {
                    let param_indicator_index =
                        tokens.get_next_meaningful_token(arg_before).unwrap();
                    let after_param_indicator_index = tokens
                        .get_next_meaningful_token(param_indicator_index)
                        .unwrap();
                    if tokens[param_indicator_index].equals((T_STRING, "null"), false)
                        && tokens[after_param_indicator_index].equals(")", true)
                    {
                        if tokens[arg_before + 1].is_whitespace(None) {
                            tokens.clear_token_and_merge_surrounding_whitespace(arg_before + 1);
                        }
                        tokens.clear_token_and_merge_surrounding_whitespace(arg_before);
                        tokens.clear_token_and_merge_surrounding_whitespace(param_indicator_index);
                        continue;
                    }
                }
                is_multiline_whitespace = is_multiline_whitespace
                    || (tokens[arg_start].is_whitespace(None)
                        && !tokens[arg_start].is_whitespace(Some(b" \t")));
                let mut override_arg_start = vec![
                    Token::new(T_WHITESPACE, indent.clone()),
                    Token::new(T_VARIABLE, b"$this".to_vec()),
                    Token::new(T_OBJECT_OPERATOR, b"->".to_vec()),
                    Token::new(T_STRING, replacement.as_bytes().to_vec()),
                    Token::char(b"("),
                ];
                let mut override_arg_before = vec![Token::char(b")"), Token::char(b";")];
                if is_multiline_whitespace {
                    let mut deeper = indent.clone();
                    deeper.extend(self.whitespaces_config.get_indent().as_bytes());
                    override_arg_start.push(Token::new(T_WHITESPACE, deeper));
                    override_arg_before.insert(0, Token::new(T_WHITESPACE, indent.clone()));
                }
                if tokens[arg_start].is_whitespace(None) {
                    tokens.override_range(arg_start, arg_start, override_arg_start);
                } else {
                    tokens.insert_at(arg_start, override_arg_start);
                }
                tokens.override_range(arg_before, arg_before, override_arg_before);
            }
            let method_name = if tokens[index].get_content() == b"expectExceptionMessageRegExp" {
                new_expected_exception_name
            } else {
                "expectException"
            };
            tokens.set(index, Token::new(T_STRING, method_name.as_bytes().to_vec()));
            index += 1;
        }
    }
}
