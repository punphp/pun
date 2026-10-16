//! PHPStan compatibility layer (Phase 2, M2.1): NEON, phpstan.neon loading the way PHPStan's DI container resolves
//! it, baselines, `ignoreErrors` and `@phpstan-ignore` comments.

pub mod baseline;
pub mod config;
pub mod di;
pub mod neon;
pub mod php;
pub mod phpfile;
pub mod schema;
pub mod value;

pub use value::{Array, Entity, Key, Value};
