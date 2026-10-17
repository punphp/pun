//! PHPStan compatibility layer (Phase 2, M2.1): NEON, phpstan.neon loading the way PHPStan's DI container resolves
//! it, baselines, `ignoreErrors` and `@phpstan-ignore` comments; built-in
//! symbols and PHPStan's type strings (Phase 2, M2.2).

pub mod baseline;
pub mod comments;
pub mod config;
pub mod di;
pub mod ignore;
pub mod neon;
pub mod php;
pub mod phpdoc;
pub mod phpfile;
pub mod resolve;
pub mod schema;
pub mod symbols;
pub mod types;
pub mod value;

pub use value::{Array, Entity, Key, Value};
