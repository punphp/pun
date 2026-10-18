//! The ported rules. Each file is one Pun rule, ported function by function from its upstream source.

mod no_and_assignment;
mod no_concat_assign_new_item;
mod no_double_negation;
mod no_negated_and;
mod no_negated_boolean_literal;
mod no_negated_group;
mod no_single_argument_sprintf;
mod no_single_item_in_array;
mod no_tautology_ternary;
mod no_unused_catch_variable;
mod no_useless_array_values;
mod prefer_array_append;
mod prefer_compound_assignment;
mod prefer_empty_array_compare;
mod prefer_func_num_args;
mod prefer_null_coalesce;
mod rector_code_quality;

pub use no_and_assignment::NoAndAssignment;
pub use no_concat_assign_new_item::NoConcatAssignNewItem;
pub use no_double_negation::NoDoubleNegation;
pub use no_negated_and::NoNegatedAnd;
pub use no_negated_boolean_literal::NoNegatedBooleanLiteral;
pub use no_negated_group::NoNegatedGroup;
pub use no_single_argument_sprintf::NoSingleArgumentSprintf;
pub use no_single_item_in_array::NoSingleItemInArray;
pub use no_tautology_ternary::NoTautologyTernary;
pub use no_unused_catch_variable::NoUnusedCatchVariable;
pub use no_useless_array_values::NoUselessArrayValues;
pub use prefer_array_append::PreferArrayAppend;
pub use prefer_compound_assignment::PreferCompoundAssignment;
pub use prefer_empty_array_compare::PreferEmptyArrayCompare;
pub use prefer_func_num_args::PreferFuncNumArgs;
pub use prefer_null_coalesce::PreferNullCoalesce;

use crate::Rule;

/// Every rule, in a fixed order.
pub fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(NoUnusedCatchVariable),
        Box::new(PreferCompoundAssignment),
        Box::new(NoAndAssignment),
        Box::new(PreferNullCoalesce),
        Box::new(PreferArrayAppend),
        Box::new(NoNegatedAnd),
        Box::new(NoConcatAssignNewItem),
        Box::new(NoNegatedBooleanLiteral),
        Box::new(NoDoubleNegation),
        Box::new(NoNegatedGroup),
        Box::new(PreferEmptyArrayCompare),
        Box::new(PreferFuncNumArgs),
        Box::new(NoUselessArrayValues),
        Box::new(NoTautologyTernary),
        Box::new(NoSingleItemInArray),
        Box::new(NoSingleArgumentSprintf),
    ]
}
