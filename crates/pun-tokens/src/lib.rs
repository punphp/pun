//! PHP-CS-Fixer's token model in Rust. See docs/tokens.md.

pub mod kind;
pub mod lexer;

pub use kind::Kind;
pub use lexer::{RawToken, tokenize};
