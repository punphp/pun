//! Ported PHP-CS-Fixer / Pint fixers. One file per fixer in src/fixers (registered by build.rs);
//! shared ports of PHP-CS-Fixer's Tokenizer\Analyzer, DocBlock, Preg etc. live in src/analyzers.

pub mod analyzers;

include!(concat!(env!("OUT_DIR"), "/fixers.rs"));

/// A fixer with its options' defaults. Every configurable fixer's `fixer()` builds it with neutral fields and lets
/// `configure(Null)` set the defaults, so they are written once (in `configure`), as `Pipeline::new` sees them.
pub(crate) fn configured(mut f: impl pun_format::Fixer + 'static) -> Box<dyn pun_format::Fixer> {
    f.configure(&serde_json::Value::Null)
        .expect("the defaults are a valid configuration");
    Box::new(f)
}
