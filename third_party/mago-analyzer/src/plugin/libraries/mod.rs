//! Library-specific providers for the PHP stdlib.

pub mod stdlib;

use crate::plugin::Plugin;

pub use stdlib::StdlibPlugin;

/// All available analyzer plugins.
pub static ALL_PLUGINS: &[&dyn Plugin] = &[&StdlibPlugin];
