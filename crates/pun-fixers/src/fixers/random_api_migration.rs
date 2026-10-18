//! Port of `PhpCsFixer\Fixer\Alias\RandomApiMigrationFixer` (v3 defaults: `Future::getV4OrV3` outside future mode).
use crate::analyzers::abstract_function_reference_fixer::{FunctionReferenceFinder, IS_RISKY};
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::{Result, bail};
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct RandomApiMigrationFixer {
    replacements: Vec<(String, String)>,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(RandomApiMigrationFixer {
        replacements: vec![],
    })
}

/// `ARGUMENT_COUNTS`.
fn argument_counts(function: &str) -> Option<&'static [usize]> {
    Some(match function {
        "mt_getrandmax" | "getrandmax" => &[0],
        "mt_rand" => &[1, 2],
        "rand" | "random_int" => &[0, 2],
        "srand" | "mt_srand" => &[0, 1],
        _ => return None,
    })
}

impl Fixer for RandomApiMigrationFixer {
    fn name(&self) -> &'static str {
        "random_api_migration"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        IS_RISKY
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["replacements"])?;
        o.check_types("replacements", &["string[]"])?;
        self.replacements = match o.php_array("replacements") {
            None => [
                ("getrandmax", "mt_getrandmax"),
                ("rand", "mt_rand"),
                ("srand", "mt_srand"),
            ]
            .iter()
            .map(|&(a, b)| (a.to_string(), b.to_string()))
            .collect(),
            Some(map) => {
                let out: Vec<(String, String)> = map
                    .into_iter()
                    .map(|(function, replacement)| {
                        (
                            function.to_string(),
                            replacement.as_str().unwrap().to_string(),
                        )
                    })
                    .collect();
                if let Some((function, _)) = out.iter().find(|(f, _)| argument_counts(f).is_none())
                {
                    bail!(
                        "[{}] Invalid configuration: Function \"{function}\" is not handled by the fixer.",
                        self.name()
                    );
                }
                out
            }
        };
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let finder = FunctionReferenceFinder::new();
        let arguments_analyzer = ArgumentsAnalyzer;
        for (function_identity, function_replacement) in &self.replacements {
            if function_identity == function_replacement {
                continue;
            }
            let mut curr_index = 0;
            while let Some((function_name, open_parenthesis, close_parenthesis)) = finder.find(
                function_identity,
                tokens,
                curr_index,
                Some(tokens.count() - 1),
            ) {
                let count =
                    arguments_analyzer.count_arguments(tokens, open_parenthesis, close_parenthesis);
                if !argument_counts(function_identity).unwrap().contains(&count) {
                    break;
                }
                curr_index = open_parenthesis;
                tokens.set(
                    function_name,
                    Token::new(T_STRING, function_replacement.as_str()),
                );
                if count == 0 && function_replacement == "random_int" {
                    tokens.insert_at(
                        curr_index + 1,
                        vec![
                            Token::new(T_LNUMBER, "0"),
                            Token::char(","),
                            Token::new(T_WHITESPACE, " "),
                            Token::new(T_STRING, "getrandmax"),
                            Token::char("("),
                            Token::char(")"),
                        ],
                    );
                    curr_index += 6;
                }
            }
        }
    }
}
