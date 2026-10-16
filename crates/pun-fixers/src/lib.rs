//! Ported PHP-CS-Fixer / Pint fixers. One file per fixer in src/fixers (registered by build.rs);
//! shared ports of PHP-CS-Fixer's Tokenizer\Analyzer, DocBlock, Preg etc. live in src/analyzers.

pub mod analyzers;

include!(concat!(env!("OUT_DIR"), "/fixers.rs"));
