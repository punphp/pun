//! Port of `PhpCsFixer\Fixer\Casing\NativeTypeDeclarationCasingFixer` (PHP 8.5 type list).
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::token::CLASSY_KINDS;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct NativeTypeDeclarationCasingFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NativeTypeDeclarationCasingFixer)
}

const TYPES: &[&[u8]] = &[
    b"array",
    b"bool",
    b"callable",
    b"float",
    b"int",
    b"iterable",
    b"object",
    b"parent",
    b"self",
    b"static",
    b"string",
    b"void",
    b"false",
    b"mixed",
    b"null",
    b"never",
    b"true",
];

impl Fixer for NativeTypeDeclarationCasingFixer {
    fn name(&self) -> &'static str {
        "native_type_declaration_casing"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        let classy_found = tokens.is_any_token_kinds_found(CLASSY_KINDS);
        // Upstream also accepts `T_CONST && classy`, which the T_STRING check already covers: a class constant's name
        // is a T_STRING.
        tokens.is_any_token_kinds_found(&[T_FUNCTION, T_FN])
            || (classy_found && tokens.is_token_kind_found(T_STRING))
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.len() {
            let token = &tokens[index];
            let lowercase_content = token.content.to_ascii_lowercase();
            if token.content == lowercase_content || !TYPES.contains(&&lowercase_content[..]) {
                continue;
            }
            let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
            if tokens[prev_index].equals("=", true)
                || tokens[prev_index].is_given_kind(&[
                    T_CASE,
                    T_OBJECT_OPERATOR,
                    T_DOUBLE_COLON,
                    T_NS_SEPARATOR,
                ])
            {
                continue;
            }
            let next_index = tokens.get_next_meaningful_token(index).unwrap();
            if tokens[next_index].equals("=", true) || tokens[next_index].is_kind(T_NS_SEPARATOR) {
                continue;
            }
            if !tokens[prev_index].is_given_kind(&[
                T_CONST,
                CT_NULLABLE_TYPE,
                CT_TYPE_ALTERNATION,
                CT_TYPE_COLON,
            ]) && !tokens[next_index].is_given_kind(&[T_VARIABLE, CT_TYPE_ALTERNATION])
            {
                continue;
            }
            // Every native type keyword is a T_STRING/T_ARRAY/T_CALLABLE/T_STATIC token, never a bare char.
            tokens.set(
                index,
                Token::new(tokens[index].id.unwrap(), lowercase_content),
            );
        }
    }
}
