//! Port of `PhpCsFixer\Fixer\Whitespace\TypesSpacesFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::{Result, bail};
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

struct TypesSpacesFixer {
    space_single: bool,
    space_multiple_catch_single: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(TypesSpacesFixer {
        space_single: false,
        space_multiple_catch_single: false,
    })
}

impl Fixer for TypesSpacesFixer {
    fn name(&self) -> &'static str {
        "types_spaces"
    }
    fn priority(&self) -> i32 {
        -1
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[CT_TYPE_ALTERNATION, CT_TYPE_INTERSECTION])
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["space", "space_multiple_catch"])?;
        let space = o.string("space", "none", &["none", "single"])?;
        let catch = match o.get("space_multiple_catch") {
            None | Some(Value::Null) => space.clone(),
            Some(Value::String(s)) if s == "none" || s == "single" => s.clone(),
            Some(v) => bail!(
                "[types_spaces] Invalid configuration: The option \"space_multiple_catch\" with value {v} is invalid. Accepted values are: \"none\", \"single\", null."
            ),
        };
        self.space_single = space == "single";
        self.space_multiple_catch_single = catch == "single";
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut token_count = tokens.count() - 1;
        let mut index = 0;
        while index < token_count {
            if tokens[index].is_given_kind(&[CT_TYPE_ALTERNATION, CT_TYPE_INTERSECTION]) {
                token_count += fix_spacing(tokens, index, self.space_single);
                index += 1;
                continue;
            }
            if tokens[index].is_kind(T_CATCH) {
                loop {
                    index = tokens
                        .get_next_token_of_kind(
                            index,
                            &[")".into(), CT_TYPE_ALTERNATION.into()],
                            true,
                        )
                        .unwrap();
                    if tokens[index].equals(")", true) {
                        break;
                    }
                    token_count += fix_spacing(tokens, index, self.space_multiple_catch_single);
                }
            }
            index += 1;
        }
    }
}

fn fix_spacing(tokens: &mut Tokens, index: usize, single_space: bool) -> usize {
    if !single_space {
        ensure_no_space(tokens, index + 1);
        ensure_no_space(tokens, index - 1);
        return 0;
    }
    let mut added_token_count = 0;
    added_token_count += ensure_single_space(tokens, index + 1, 0);
    added_token_count += ensure_single_space(tokens, index - 1, 1);
    added_token_count
}

fn ensure_single_space(tokens: &mut Tokens, index: usize, offset: usize) -> usize {
    if !tokens[index].is_whitespace(None) {
        tokens.insert_slices(BTreeMap::from([(
            index + offset,
            vec![Token::new(T_WHITESPACE, " ")],
        )]));
        return 1;
    }
    if tokens[index].content != b" " && !Preg::r#match("/\\R/", &tokens[index].content) {
        tokens.set(index, Token::new(T_WHITESPACE, " "));
    }
    0
}

fn ensure_no_space(tokens: &mut Tokens, index: usize) {
    if tokens[index].is_whitespace(None) && !Preg::r#match("/\\R/", &tokens[index].content) {
        tokens.clear_at(index);
    }
}
