//! The ported rules. Each file is one Pun rule, ported function by function from its upstream source.

mod no_and_assignment;
mod no_and_true;
mod no_cast_in_concat;
mod no_concat_assign_new_item;
mod no_double_negation;
mod no_double_self_assignment;
mod no_duplicate_array_key;
mod no_empty_block;
mod no_negated_and;
mod no_negated_boolean_literal;
mod no_negated_group;
mod no_null_property_init;
mod no_redundant_final_const;
mod no_return_in_constructor;
mod no_single_argument_sprintf;
mod no_single_item_in_array;
mod no_tautology_ternary;
mod no_unused_catch_variable;
mod no_unused_closure_variable;
mod no_unused_foreach_key;
mod no_useless_array_values;
mod no_useless_block;
mod no_useless_catch;
mod no_useless_continue;
mod php_parser;
mod prefer_array_append;
mod prefer_compound_assignment;
mod prefer_empty_array_compare;
mod prefer_func_num_args;
mod prefer_null_coalesce;
mod rector_code_quality;
mod used_variable;

pub use no_and_assignment::NoAndAssignment;
pub use no_and_true::NoAndTrue;
pub use no_cast_in_concat::NoCastInConcat;
pub use no_concat_assign_new_item::NoConcatAssignNewItem;
pub use no_double_negation::NoDoubleNegation;
pub use no_double_self_assignment::NoDoubleSelfAssignment;
pub use no_duplicate_array_key::NoDuplicateArrayKey;
pub use no_empty_block::NoEmptyBlock;
pub use no_negated_and::NoNegatedAnd;
pub use no_negated_boolean_literal::NoNegatedBooleanLiteral;
pub use no_negated_group::NoNegatedGroup;
pub use no_null_property_init::NoNullPropertyInit;
pub use no_redundant_final_const::NoRedundantFinalConst;
pub use no_return_in_constructor::NoReturnInConstructor;
pub use no_single_argument_sprintf::NoSingleArgumentSprintf;
pub use no_single_item_in_array::NoSingleItemInArray;
pub use no_tautology_ternary::NoTautologyTernary;
pub use no_unused_catch_variable::NoUnusedCatchVariable;
pub use no_unused_closure_variable::NoUnusedClosureVariable;
pub use no_unused_foreach_key::NoUnusedForeachKey;
pub use no_useless_array_values::NoUselessArrayValues;
pub use no_useless_block::NoUselessBlock;
pub use no_useless_catch::NoUselessCatch;
pub use no_useless_continue::NoUselessContinue;
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
        Box::new(NoAndTrue),
        Box::new(NoCastInConcat),
        Box::new(NoUselessContinue),
        Box::new(NoEmptyBlock),
        Box::new(NoUselessCatch),
        Box::new(NoDoubleSelfAssignment),
        Box::new(NoDuplicateArrayKey),
        Box::new(NoRedundantFinalConst),
        Box::new(NoNullPropertyInit),
        Box::new(NoUnusedClosureVariable),
        Box::new(NoUnusedForeachKey),
        Box::new(NoReturnInConstructor),
        Box::new(NoUselessBlock),
    ]
}
