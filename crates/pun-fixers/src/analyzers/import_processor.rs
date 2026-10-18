//! Port of `PhpCsFixer\Tokenizer\Processor\ImportProcessor`.
use pun_format::WhitespacesFixerConfig;
use pun_tokens::{Kind::*, Token, Tokens};

pub struct ImportProcessor {
    whitespaces_config: WhitespacesFixerConfig,
}

/// `const` / `class` / `function`.
pub type ImportType = &'static str;

impl ImportProcessor {
    pub fn new(whitespaces_config: WhitespacesFixerConfig) -> Self {
        ImportProcessor { whitespaces_config }
    }

    /// `imports`: `[type => names]` in PHP array order (each type's names are sorted here, like upstream).
    pub fn insert_imports(
        &self,
        tokens: &mut Tokens,
        imports: &[(ImportType, Vec<Vec<u8>>)],
        mut at_index: usize,
    ) {
        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
        if let Some(prev_index) = tokens.get_prev_meaningful_token(at_index)
            && tokens[prev_index].is_kind(T_OPEN_TAG_WITH_ECHO)
        {
            let t = Tokens::from_code(b"<?php\n?>");
            tokens.insert_at(prev_index, t.iter().cloned().collect());
            at_index = prev_index + 1;
        }
        if !tokens[at_index].is_whitespace(None) || !tokens[at_index].get_content().contains(&b'\n')
        {
            tokens.insert_at(at_index, vec![Token::new(T_WHITESPACE, line_ending)]);
        }
        for (ty, type_imports) in imports {
            let mut type_imports = type_imports.clone();
            // ponytail: PHP sort() compares numeric strings numerically; import names never are numeric.
            type_imports.sort();
            let mut items = Vec::new();
            for name in &type_imports {
                items.push(Token::new(T_WHITESPACE, line_ending));
                items.push(Token::new(T_USE, "use"));
                items.push(Token::new(T_WHITESPACE, " "));
                if *ty == "const" {
                    items.push(Token::new(CT_CONST_IMPORT, "const"));
                    items.push(Token::new(T_WHITESPACE, " "));
                } else if *ty == "function" {
                    items.push(Token::new(CT_FUNCTION_IMPORT, "function"));
                    items.push(Token::new(T_WHITESPACE, " "));
                }
                items.extend(pun_tokens::transformers::tokenize_name(name));
                items.push(Token::char(";"));
            }
            tokens.insert_at(at_index, items);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected: the real ImportProcessor (WhitespacesFixerConfig('    ', "\r\n")) on the same input.
    #[test]
    fn matches_php() {
        let imports: &[(ImportType, Vec<Vec<u8>>)] = &[
            ("const", vec![b"B\\C".to_vec(), b"A\\B".to_vec()]),
            ("function", vec![b"f\\g".to_vec()]),
            ("class", vec![b"Z\\Y".to_vec(), b"X".to_vec()]),
        ];
        let uses =
            "\r\nuse X;\r\nuse Z\\Y;\r\nuse function f\\g;\r\nuse const A\\B;\r\nuse const B\\C;";
        for (code, at, expected) in [
            ("<?= 1 ?>\n", 1, format!("<?php\n{uses}\r\n?><?= 1 ?>\n")),
            (
                "<?php\nnamespace A;\necho 1;\n",
                5,
                format!("<?php\nnamespace A;{uses}\necho 1;\n"),
            ),
            (
                "<?php\nnamespace A;echo 1;\n",
                5,
                format!("<?php\nnamespace A;{uses}\r\necho 1;\n"),
            ),
        ] {
            let mut tokens = Tokens::from_code(code.as_bytes());
            ImportProcessor::new(WhitespacesFixerConfig::new("    ", "\r\n").unwrap())
                .insert_imports(&mut tokens, imports, at);
            assert_eq!(
                String::from_utf8(tokens.generate_code()).unwrap(),
                expected,
                "{code:?}"
            );
        }
    }
}
