//! Port of `PhpCsFixer\Fixer\Basic\NoMultipleStatementsPerLineFixer`.
use crate::analyzers::indentation_trait::get_line_indentation;
use crate::analyzers::preg::Preg;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Tokens};
use std::path::Path;

#[derive(Default)]
struct NoMultipleStatementsPerLineFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoMultipleStatementsPerLineFixer::default())
}

impl Fixer for NoMultipleStatementsPerLineFixer {
    fn name(&self) -> &'static str {
        "no_multiple_statements_per_line"
    }
    fn priority(&self) -> i32 {
        // Must run before BracesPositionFixer, CurlyBracesPositionFixer.
        // Must run after ControlStructureBracesFixer, NoEmptyStatementFixer, YieldFromArrayToYieldsFixer.
        -1
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(";")
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = 1;
        let mut max = tokens.count() - 1;
        while index < max {
            if tokens[index].is_kind(T_FOR) {
                index = tokens.find_block_end(
                    Tokens::BLOCK_TYPE_PARENTHESIS,
                    tokens
                        .get_next_token_of_kind(index, &["(".into()], true)
                        .unwrap(),
                );
                index += 1;
                continue;
            }
            if tokens[index].is_kind(CT_PROPERTY_HOOK_BRACE_OPEN) {
                index = tokens.find_block_end(Tokens::BLOCK_TYPE_PROPERTY_HOOK, index);
                index += 1;
                continue;
            }
            if !tokens[index].equals(";", true) {
                index += 1;
                continue;
            }
            let mut next_index = index + 1;
            while next_index < max {
                let token = &tokens[next_index];
                if token.is_whitespace(None) || token.is_comment() {
                    if Preg::r#match("/\\R/", token.get_content()) {
                        break;
                    }
                    next_index += 1;
                    continue;
                }
                if !token.equals_any(
                    &[
                        "}".into(),
                        T_CLOSE_TAG.into(),
                        T_ENDIF.into(),
                        T_ENDFOR.into(),
                        T_ENDSWITCH.into(),
                        T_ENDWHILE.into(),
                        T_ENDFOREACH.into(),
                    ],
                    true,
                ) {
                    let mut whitespace_index = index;
                    loop {
                        whitespace_index += 1;
                        if !tokens[whitespace_index].is_comment() {
                            break;
                        }
                    }
                    let newline = [
                        self.whitespaces_config.get_line_ending().as_bytes(),
                        &get_line_indentation(tokens, index),
                    ]
                    .concat();
                    if tokens.ensure_whitespace_at_index(whitespace_index, 0, &newline) {
                        max += 1;
                    }
                }
                break;
            }
            index += 1;
        }
    }
}
