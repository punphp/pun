//! The ported rules. Each file is one Pun rule, ported function by function from its upstream source.

mod no_unused_catch_variable;
mod prefer_compound_assignment;

pub use no_unused_catch_variable::NoUnusedCatchVariable;
pub use prefer_compound_assignment::PreferCompoundAssignment;

use crate::Rule;

/// Every rule, in a fixed order.
pub fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(NoUnusedCatchVariable),
        Box::new(PreferCompoundAssignment),
    ]
}
