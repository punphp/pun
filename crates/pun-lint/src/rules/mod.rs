//! The ported rules. Each file is one Pun rule, ported function by function from its upstream source.

mod add_csv_escape_argument;
mod mago_helpers;
mod mago_restricted;
mod mago_calls;
mod mago_comments;
mod mago_extension_functions;
mod mago_style;
mod no_boolean_flag_parameter;
mod no_else_clause;
mod no_empty_file;
mod no_excessive_cyclomatic;
mod no_excessive_halstead;
mod no_excessive_kan_defect;
mod no_excessive_members;
mod no_excessive_nesting;
mod no_excessive_parameters;
mod no_goto;
mod no_side_effects_with_declarations;
mod prefer_named_literal_argument;
mod require_interpolation_style;
mod require_naming_convention;
mod require_phpdoc;
mod no_unused_promoted_property;
mod require_parent_visibility;
mod add_datetime_property_type;
mod no_dead_catch;
mod prefer_class_keyword;
mod prefer_class_name_constant;
mod no_unused_constructor_param;
mod no_null_default_argument;
mod add_param_type_sprintf;
mod class_method;
mod laravel;
mod pest;
mod phpunit;
mod lint_b_doc;
mod lint_b_early_exit;
mod lint_b_foreach;
mod lint_b_helpers;
mod lint_b_names;
mod lint_b_privatize;
mod lint_b_security;
mod lint_b_serialize;
mod mago_const_value;
mod mago_usage;
mod no_always_false_condition;
mod no_always_true_if;
mod no_always_true_instanceof;
mod no_always_true_instanceof_assert;
mod no_and_assignment;
mod no_and_true;
mod no_array_accumulation_in_loop;
mod no_assign_in_argument;
mod no_assign_in_condition;
mod no_assign_in_ternary_branch;
mod no_bool_ternary;
mod no_boolean_literal_compare;
mod no_call_user_func_method;
mod no_cast_in_concat;
mod no_compact;
mod no_concat_assign_new_item;
mod no_condition_exact_return;
mod no_constant_condition;
mod no_constant_via_variable;
mod no_dead_php_version_check;
mod no_debug_calls;
mod no_double_negation;
mod no_double_self_assignment;
mod no_duplicate_array_key;
mod no_duplicate_case_body;
mod no_duplicate_match_arm;
mod no_dynamic_member;
mod no_extra_arguments;
mod no_dynamic_name;
mod no_else_after_return;
mod no_empty;
mod no_empty_block;
mod no_empty_catch;
mod no_empty_loop;
mod no_empty_method;
mod no_escaped_quotes;
mod no_eval;
mod no_explicit_constructor_call;
mod no_false_ternary;
mod no_ffi;
mod no_finfo_buffer_context;
mod no_for_init_overwrite;
mod no_future_php_version_check;
mod no_global;
mod no_hash_emoji;
mod no_identical_branches;
mod no_implode_ternary;
mod no_insecure_comparison;
mod no_invalid_open_tag;
mod no_iterator_to_array_in_foreach;
mod no_literal_password;
mod no_loop_variable_overwrite;
mod no_loose_equality;
mod no_mirror_assignment;
mod no_missing_format_argument;
mod no_mixed_overruled_by_native;
mod no_multi_assign;
mod no_negated_and;
mod no_negated_boolean_literal;
mod no_negated_comparison;
mod no_negated_condition;
mod no_negated_group;
mod no_nested_ternary;
mod no_null_debug_info;
mod no_null_property_init;
mod no_null_tag_value;
mod no_out_of_range_chr;
mod no_overwritten_assignment;
mod no_parameter_shadowing;
mod no_parent_call_without_parent;
mod no_protected_constant_in_final;
mod no_protected_method_in_final;
mod no_protected_property_in_final;
mod no_redundant_condition;
mod no_redundant_default_argument;
mod no_redundant_final_const;
mod no_invalid_var_tag;
mod no_phpdoc_class_case_mismatch;
mod no_redundant_foreach_guard;
mod no_redundant_null_check;
mod no_redundant_promoted_assign;
mod no_redundant_property_default;
mod no_redundant_readonly_property;
mod no_redundant_write_visibility;
mod no_request_variable;
mod no_required_after_optional;
mod no_return_in_constructor;
mod no_short_ternary;
mod no_single_argument_sprintf;
mod no_single_case_switch;
mod no_single_item_in_array;
mod no_sole_value_sprintf;
mod no_static_constructor_call;
mod no_swapped_explode_arguments;
mod no_tainted_data;
mod no_tautology_ternary;
mod no_typed_property_instanceof;
mod no_underscore_class;
mod no_unreachable_code;
mod no_unreachable_loop;
mod no_unsafe_finally;
mod no_unused_closure_variable;
mod no_unused_expression;
mod no_unused_foreach_key;
mod no_unused_global;
mod no_unused_label;
mod no_unused_private_constant;
mod no_unused_private_method;
mod no_unused_private_method_param;
mod no_unused_private_property;
mod no_unused_public_method_param;
mod no_unused_static;
mod no_unused_test_method_param;
mod no_useless_array_values;
mod no_useless_block;
mod no_useless_cast;
mod no_useless_catch;
mod no_useless_condition;
mod no_useless_constructor;
mod no_useless_continue;
mod no_useless_filter_var;
mod no_useless_math;
mod no_useless_override;
mod no_useless_readonly;
mod no_useless_ternary;
mod no_useless_variable;
mod no_variable_variable;
mod no_void_on_magic_method;
mod no_void_reference_return;
mod php_parser;
mod phpstan_describe;
mod phpdoc_tag;
mod prefer_absolute_include;
mod prefer_array_all;
mod prefer_array_any;
mod prefer_array_append;
mod prefer_array_find;
mod prefer_array_find_key;
mod prefer_array_first_last;
mod prefer_array_literal;
mod prefer_boolean_and_or;
mod prefer_catch_name_from_type;
mod prefer_clamp;
mod prefer_coalesce_chain;
mod prefer_coalesce_over_empty_ternary;
mod prefer_coalesce_over_foreach;
mod prefer_compound_assignment;
mod prefer_early_continue;
mod prefer_early_return;
mod prefer_early_return_of_variable;
mod prefer_early_return_over_prepared_value;
mod prefer_empty_array_compare;
mod prefer_empty_array_identical;
mod prefer_empty_string_identical;
mod prefer_func_num_args;
mod prefer_in_array;
mod prefer_in_array_over_foreach;
mod prefer_in_array_over_search;
mod prefer_inline_array_return;
mod no_useless_var_tag;
mod no_useless_param_tag;
mod no_useless_return_tag;
mod no_invalid_return_tag;
mod no_useless_phpdoc_type;
mod no_redundant_nullable_check;
mod no_unused_assignment;
mod prefer_instance_method;
mod prefer_instanceof_stringable;
mod prefer_min_max;
mod prefer_new_self;
mod prefer_not_in_array;
mod prefer_null_coalesce;
mod prefer_ord_first_byte;
mod prefer_php_version_id;
mod prefer_property_default;
mod prefer_property_exists;
mod prefer_property_over_local_getter;
mod prefer_returned_condition;
mod prefer_round_mode_enum;
mod prefer_self_over_static;
mod prefer_serialize_method;
mod prefer_simple_condition;
mod prefer_spread;
mod prefer_unserialize_method;
mod prefer_variadic_param;
mod rector_batch5;
mod rector_code_quality;
mod rector_conditions;
mod rector_stmts;
mod refactor;
mod require_assign_set_padding;
mod require_attribute_argument_order;
mod require_is_a_allow_string;
mod require_named_argument_order;
mod require_preg_quote_delimiter;
mod require_consistent_return;
mod require_previous_exception;
mod require_statement_padding;
mod require_strict_function_call;
mod require_strict_array_search;
mod require_array_filter_callback;
mod require_strict_in_array;
mod side_effect;
mod stmt_splice;
mod syntax_types;
mod types;
mod used_variable;

pub use add_csv_escape_argument::AddCsvEscapeArgument;
pub use mago_calls::{
    NoAmbiguousConstant, NoAmbiguousFunction, NoIniSet, NoSprintfConcat, PreferCreateFromTimestamp,
    RequireAssertDescription,
};
pub use mago_comments::{NoInvalidPhpdoc, RequireTodoReference};
pub use mago_restricted::{NoRestrictedFunctions, NoRestrictedInstantiation};
pub use mago_style::{
    NoInlineHtml, NoIsset, NoMultipleClassesPerFile, PreferWhileLoop, RequireNamespace, RequireSortedIntegerKeys,
};
pub use no_boolean_flag_parameter::NoBooleanFlagParameter;
pub use no_else_clause::NoElseClause;
pub use no_empty_file::NoEmptyFile;
pub use no_excessive_cyclomatic::NoExcessiveCyclomatic;
pub use no_excessive_halstead::NoExcessiveHalstead;
pub use no_excessive_kan_defect::NoExcessiveKanDefect;
pub use no_excessive_members::{NoExcessiveEnumCases, NoExcessiveMethods, NoExcessiveProperties};
pub use no_excessive_nesting::NoExcessiveNesting;
pub use no_excessive_parameters::NoExcessiveParameters;
pub use no_goto::NoGoto;
pub use no_side_effects_with_declarations::NoSideEffectsWithDeclarations;
pub use prefer_named_literal_argument::PreferNamedLiteralArgument;
pub use require_interpolation_style::RequireInterpolationStyle;
pub use require_naming_convention::RequireNamingConvention;
pub use require_phpdoc::RequirePhpdoc;
pub use no_unused_promoted_property::NoUnusedPromotedProperty;
pub use require_parent_visibility::RequireParentVisibility;
pub use add_datetime_property_type::AddDatetimePropertyType;
pub use no_dead_catch::NoDeadCatch;
pub use prefer_class_keyword::PreferClassKeyword;
pub use prefer_class_name_constant::PreferClassNameConstant;
pub use no_unused_constructor_param::NoUnusedConstructorParam;
pub use no_null_default_argument::{NoNullDefaultArgument, NoNullDefaultNamedArgument};
pub use add_param_type_sprintf::AddParamTypeSprintf;
pub use laravel::*;
pub use pest::*;
pub use phpunit::*;
pub use no_always_false_condition::NoAlwaysFalseCondition;
pub use no_always_true_if::NoAlwaysTrueIf;
pub use no_always_true_instanceof::NoAlwaysTrueInstanceof;
pub use no_always_true_instanceof_assert::NoAlwaysTrueInstanceofAssert;
pub use no_and_assignment::NoAndAssignment;
pub use no_and_true::NoAndTrue;
pub use no_array_accumulation_in_loop::NoArrayAccumulationInLoop;
pub use no_assign_in_argument::NoAssignInArgument;
pub use no_assign_in_condition::NoAssignInCondition;
pub use no_assign_in_ternary_branch::NoAssignInTernaryBranch;
pub use no_bool_ternary::NoBoolTernary;
pub use no_boolean_literal_compare::NoBooleanLiteralCompare;
pub use no_call_user_func_method::NoCallUserFuncMethod;
pub use no_cast_in_concat::NoCastInConcat;
pub use no_compact::NoCompact;
pub use no_concat_assign_new_item::NoConcatAssignNewItem;
pub use no_condition_exact_return::NoConditionExactReturn;
pub use no_constant_condition::NoConstantCondition;
pub use no_constant_via_variable::NoConstantViaVariable;
pub use no_dead_php_version_check::NoDeadPhpVersionCheck;
pub use no_debug_calls::NoDebugCalls;
pub use no_double_negation::NoDoubleNegation;
pub use no_double_self_assignment::NoDoubleSelfAssignment;
pub use no_duplicate_array_key::NoDuplicateArrayKey;
pub use no_duplicate_case_body::NoDuplicateCaseBody;
pub use no_duplicate_match_arm::NoDuplicateMatchArm;
pub use no_dynamic_member::{
    NoDynamicMethodCall, NoDynamicMethodCallable, NoDynamicProperty, NoDynamicStaticCall,
    NoDynamicStaticCallable, NoDynamicStaticProperty,
};
pub use no_dynamic_name::NoDynamicName;
pub use no_extra_arguments::NoExtraArguments;
pub use no_else_after_return::NoElseAfterReturn;
pub use no_empty::NoEmpty;
pub use no_empty_block::NoEmptyBlock;
pub use no_empty_catch::NoEmptyCatch;
pub use no_empty_loop::NoEmptyLoop;
pub use no_empty_method::NoEmptyMethod;
pub use no_escaped_quotes::NoEscapedQuotes;
pub use no_eval::NoEval;
pub use no_explicit_constructor_call::NoExplicitConstructorCall;
pub use no_false_ternary::NoFalseTernary;
pub use no_ffi::NoFfi;
pub use no_finfo_buffer_context::NoFinfoBufferContext;
pub use no_for_init_overwrite::NoForInitOverwrite;
pub use no_future_php_version_check::NoFuturePhpVersionCheck;
pub use no_global::NoGlobal;
pub use no_hash_emoji::NoHashEmoji;
pub use no_identical_branches::NoIdenticalBranches;
pub use no_implode_ternary::NoImplodeTernary;
pub use no_insecure_comparison::NoInsecureComparison;
pub use no_invalid_open_tag::NoInvalidOpenTag;
pub use no_iterator_to_array_in_foreach::NoIteratorToArrayInForeach;
pub use no_literal_password::NoLiteralPassword;
pub use no_loop_variable_overwrite::NoLoopVariableOverwrite;
pub use no_loose_equality::NoLooseEquality;
pub use no_mirror_assignment::NoMirrorAssignment;
pub use no_missing_format_argument::NoMissingFormatArgument;
pub use no_mixed_overruled_by_native::NoMixedOverruledByNative;
pub use no_multi_assign::NoMultiAssign;
pub use no_negated_and::NoNegatedAnd;
pub use no_negated_boolean_literal::NoNegatedBooleanLiteral;
pub use no_negated_comparison::NoNegatedComparison;
pub use no_negated_condition::NoNegatedCondition;
pub use no_negated_group::NoNegatedGroup;
pub use no_nested_ternary::NoNestedTernary;
pub use no_null_debug_info::NoNullDebugInfo;
pub use no_null_property_init::NoNullPropertyInit;
pub use no_null_tag_value::NoNullTagValue;
pub use no_out_of_range_chr::NoOutOfRangeChr;
pub use no_overwritten_assignment::NoOverwrittenAssignment;
pub use no_parameter_shadowing::NoParameterShadowing;
pub use no_parent_call_without_parent::NoParentCallWithoutParent;
pub use no_protected_constant_in_final::NoProtectedConstantInFinal;
pub use no_protected_method_in_final::NoProtectedMethodInFinal;
pub use no_protected_property_in_final::NoProtectedPropertyInFinal;
pub use no_redundant_condition::NoRedundantCondition;
pub use no_redundant_default_argument::NoRedundantDefaultArgument;
pub use no_redundant_final_const::NoRedundantFinalConst;
pub use no_invalid_var_tag::NoInvalidVarTag;
pub use no_phpdoc_class_case_mismatch::NoPhpdocClassCaseMismatch;
pub use no_redundant_foreach_guard::NoRedundantForeachGuard;
pub use no_redundant_null_check::NoRedundantNullCheck;
pub use no_redundant_promoted_assign::NoRedundantPromotedAssign;
pub use no_redundant_property_default::NoRedundantPropertyDefault;
pub use no_redundant_readonly_property::NoRedundantReadonlyProperty;
pub use no_redundant_write_visibility::NoRedundantWriteVisibility;
pub use no_request_variable::NoRequestVariable;
pub use no_required_after_optional::NoRequiredAfterOptional;
pub use no_return_in_constructor::NoReturnInConstructor;
pub use no_short_ternary::NoShortTernary;
pub use no_single_argument_sprintf::NoSingleArgumentSprintf;
pub use no_single_case_switch::NoSingleCaseSwitch;
pub use no_single_item_in_array::NoSingleItemInArray;
pub use no_sole_value_sprintf::NoSoleValueSprintf;
pub use no_static_constructor_call::NoStaticConstructorCall;
pub use no_swapped_explode_arguments::NoSwappedExplodeArguments;
pub use no_tainted_data::NoTaintedData;
pub use no_tautology_ternary::NoTautologyTernary;
pub use no_typed_property_instanceof::NoTypedPropertyInstanceof;
pub use no_underscore_class::NoUnderscoreClass;
pub use no_unreachable_code::NoUnreachableCode;
pub use no_unreachable_loop::NoUnreachableLoop;
pub use no_unsafe_finally::NoUnsafeFinally;
pub use no_unused_closure_variable::NoUnusedClosureVariable;
pub use no_unused_expression::NoUnusedExpression;
pub use no_unused_foreach_key::NoUnusedForeachKey;
pub use no_unused_global::NoUnusedGlobal;
pub use no_unused_label::NoUnusedLabel;
pub use no_unused_private_constant::NoUnusedPrivateConstant;
pub use no_unused_private_method::NoUnusedPrivateMethod;
pub use no_unused_private_method_param::NoUnusedPrivateMethodParam;
pub use no_unused_private_property::NoUnusedPrivateProperty;
pub use no_unused_public_method_param::NoUnusedPublicMethodParam;
pub use no_unused_static::NoUnusedStatic;
pub use no_unused_test_method_param::NoUnusedTestMethodParam;
pub use no_useless_array_values::NoUselessArrayValues;
pub use no_useless_block::NoUselessBlock;
pub use no_useless_cast::NoUselessCast;
pub use no_useless_catch::NoUselessCatch;
pub use no_useless_condition::NoUselessCondition;
pub use no_useless_constructor::NoUselessConstructor;
pub use no_useless_continue::NoUselessContinue;
pub use no_useless_filter_var::NoUselessFilterVar;
pub use no_useless_math::NoUselessMath;
pub use no_useless_override::NoUselessOverride;
pub use no_useless_readonly::NoUselessReadonly;
pub use no_useless_ternary::NoUselessTernary;
pub use no_useless_variable::NoUselessVariable;
pub use no_variable_variable::NoVariableVariable;
pub use no_void_on_magic_method::NoVoidOnMagicMethod;
pub use no_void_reference_return::NoVoidReferenceReturn;
pub use prefer_absolute_include::PreferAbsoluteInclude;
pub use prefer_array_all::PreferArrayAll;
pub use prefer_array_any::PreferArrayAny;
pub use prefer_array_append::PreferArrayAppend;
pub use prefer_array_find::PreferArrayFind;
pub use prefer_array_find_key::PreferArrayFindKey;
pub use prefer_array_first_last::PreferArrayFirstLast;
pub use prefer_array_literal::PreferArrayLiteral;
pub use prefer_boolean_and_or::PreferBooleanAndOr;
pub use prefer_catch_name_from_type::PreferCatchNameFromType;
pub use prefer_clamp::PreferClamp;
pub use prefer_coalesce_chain::PreferCoalesceChain;
pub use prefer_coalesce_over_empty_ternary::PreferCoalesceOverEmptyTernary;
pub use prefer_coalesce_over_foreach::PreferCoalesceOverForeach;
pub use prefer_compound_assignment::PreferCompoundAssignment;
pub use prefer_early_continue::PreferEarlyContinue;
pub use prefer_early_return::PreferEarlyReturn;
pub use prefer_early_return_of_variable::PreferEarlyReturnOfVariable;
pub use prefer_early_return_over_prepared_value::PreferEarlyReturnOverPreparedValue;
pub use prefer_empty_array_compare::PreferEmptyArrayCompare;
pub use prefer_empty_array_identical::PreferEmptyArrayIdentical;
pub use prefer_empty_string_identical::PreferEmptyStringIdentical;
pub use prefer_func_num_args::PreferFuncNumArgs;
pub use prefer_in_array::PreferInArray;
pub use prefer_in_array_over_foreach::PreferInArrayOverForeach;
pub use prefer_in_array_over_search::PreferInArrayOverSearch;
pub use prefer_inline_array_return::PreferInlineArrayReturn;
pub use no_useless_var_tag::NoUselessVarTag;
pub use no_useless_param_tag::NoUselessParamTag;
pub use no_useless_return_tag::NoUselessReturnTag;
pub use no_invalid_return_tag::NoInvalidReturnTag;
pub use no_useless_phpdoc_type::NoUselessPhpdocType;
pub use no_redundant_nullable_check::NoRedundantNullableCheck;
pub use no_unused_assignment::NoUnusedAssignment;
pub use prefer_instance_method::PreferInstanceMethod;
pub use prefer_instanceof_stringable::PreferInstanceofStringable;
pub use prefer_min_max::PreferMinMax;
pub use prefer_new_self::PreferNewSelf;
pub use prefer_not_in_array::PreferNotInArray;
pub use prefer_null_coalesce::PreferNullCoalesce;
pub use prefer_ord_first_byte::PreferOrdFirstByte;
pub use prefer_php_version_id::PreferPhpVersionId;
pub use prefer_property_default::PreferPropertyDefault;
pub use prefer_property_exists::PreferPropertyExists;
pub use prefer_property_over_local_getter::PreferPropertyOverLocalGetter;
pub use prefer_returned_condition::PreferReturnedCondition;
pub use prefer_round_mode_enum::PreferRoundModeEnum;
pub use prefer_self_over_static::PreferSelfOverStatic;
pub use prefer_serialize_method::PreferSerializeMethod;
pub use prefer_simple_condition::PreferSimpleCondition;
pub use prefer_spread::PreferSpread;
pub use prefer_unserialize_method::PreferUnserializeMethod;
pub use prefer_variadic_param::PreferVariadicParam;
pub use require_assign_set_padding::RequireAssignSetPadding;
pub use require_attribute_argument_order::RequireAttributeArgumentOrder;
pub use require_is_a_allow_string::RequireIsAAllowString;
pub use require_named_argument_order::RequireNamedArgumentOrder;
pub use require_preg_quote_delimiter::RequirePregQuoteDelimiter;
pub use require_consistent_return::RequireConsistentReturn;
pub use require_previous_exception::RequirePreviousException;
pub use require_statement_padding::RequireStatementPadding;
pub use require_strict_function_call::RequireStrictFunctionCall;
pub use require_strict_array_search::RequireStrictArraySearch;
pub use require_array_filter_callback::RequireArrayFilterCallback;
pub use require_strict_in_array::RequireStrictInArray;
pub use refactor::*;
pub use types::*;

use crate::Rule;

/// The rules outside `lint/types/` that read classes declared in other files (parents, interfaces, traits), for
/// which the CLI scans the project as it does for the types rules.
pub const PROJECT_RULES: &[&str] = &[
    "lint/dead-code/no-empty-method",
    "lint/dead-code/no-unused-private-method",
    "lint/dead-code/no-useless-constructor",
    "lint/dead-code/no-useless-override",
    "lint/dead-code/no-redundant-default-argument",
    // rector-laravel's attribute rules ask whether the class extends a framework class
    "lint/laravel/prefer-aliases-attribute",
    "lint/laravel/prefer-appends-attribute",
    "lint/laravel/prefer-backoff-attribute",
    "lint/laravel/prefer-collected-by-attribute",
    "lint/laravel/prefer-collects-attribute",
    "lint/laravel/prefer-command-hidden-attribute",
    "lint/laravel/prefer-connection-attribute",
    "lint/laravel/prefer-date-format-attribute",
    "lint/laravel/prefer-delay-attribute",
    "lint/laravel/prefer-delete-when-missing-models-attribute",
    "lint/laravel/prefer-description-attribute",
    "lint/laravel/prefer-error-bag-attribute",
    "lint/laravel/prefer-fail-on-timeout-attribute",
    "lint/laravel/prefer-fillable-attribute",
    "lint/laravel/prefer-guarded-attribute",
    "lint/laravel/prefer-help-attribute",
    "lint/laravel/prefer-hidden-attribute",
    "lint/laravel/prefer-job-connection-attribute",
    "lint/laravel/prefer-max-exceptions-attribute",
    "lint/laravel/prefer-preserve-keys-attribute",
    "lint/laravel/prefer-queue-attribute",
    "lint/laravel/prefer-signature-attribute",
    "lint/laravel/prefer-stop-on-first-failure-attribute",
    "lint/laravel/prefer-table-attribute",
    "lint/laravel/prefer-timeout-attribute",
    "lint/laravel/prefer-touches-attribute",
    "lint/laravel/prefer-tries-attribute",
    "lint/laravel/prefer-unique-for-attribute",
    "lint/laravel/prefer-visible-attribute",
    "lint/laravel/prefer-without-incrementing-attribute",
    "lint/laravel/prefer-without-timestamps-attribute",
    "lint/laravel-12/prefer-scope-attribute",
    "lint/complexity/prefer-instance-method",
    "lint/complexity/require-previous-exception",
    "lint/dead-code/no-useless-filter-var",
    "lint/dead-code/no-useless-math",
    "lint/dead-code/no-useless-cast",
    "lint/dead-code/no-always-true-instanceof",
    "lint/dead-code/no-typed-property-instanceof",
    "lint/dead-code/no-redundant-foreach-guard",
    "lint/dead-code/no-always-false-condition",
    "lint/dead-code/no-always-true-if",
    "lint/dead-code/no-always-true-instanceof-assert",
    "lint/complexity/no-constant-via-variable",
    "lint/complexity/require-consistent-return",
    "lint/complexity/no-phpdoc-class-case-mismatch",
    "lint/complexity/prefer-property-exists",
    "lint/control-flow/prefer-returned-condition",
    "lint/dead-code/no-invalid-var-tag",
    "lint/complexity/prefer-inline-array-return",
    "lint/phpdoc/no-useless-var",
    "lint/phpdoc/no-useless-param",
    "lint/phpdoc/no-useless-return",
    "lint/dead-code/no-invalid-return-tag",
    "lint/dead-code/no-useless-phpdoc-type",
    "lint/complexity/no-redundant-nullable-check",
    "lint/dead-code/no-unused-assignment",
];

/// Every rule, in a fixed order.
pub fn all() -> Vec<Box<dyn Rule>> {
    let mut rules = laravel::all();
    rules.extend(types::all());
    rules.extend(pest::all());
    rules.extend(phpunit::all());
    rules.extend(refactor::all());
    rules.extend(ungrouped());
    rules
}

/// The rules outside the group modules.
fn ungrouped() -> Vec<Box<dyn Rule>> {
    vec![
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
        Box::new(NoUnreachableCode),
        Box::new(NoMirrorAssignment),
        Box::new(NoRedundantPromotedAssign),
        Box::new(NoDuplicateCaseBody),
        Box::new(NoIdenticalBranches),
        Box::new(NoElseAfterReturn),
        Box::new(PreferEarlyReturn),
        Box::new(PreferEarlyReturnOverPreparedValue),
        Box::new(PreferEarlyReturnOfVariable),
        Box::new(PreferInArray),
        Box::new(PreferNotInArray),
        Box::new(NoRedundantNullCheck),
        Box::new(PreferSimpleCondition),
        Box::new(PreferInArrayOverSearch),
        Box::new(NoAssignInTernaryBranch),
        Box::new(NoSingleCaseSwitch),
        Box::new(NoImplodeTernary),
        Box::new(PreferNewSelf),
        Box::new(PreferArrayLiteral),
        Box::new(NoRedundantReadonlyProperty),
        Box::new(PreferAbsoluteInclude),
        Box::new(PreferBooleanAndOr),
        Box::new(NoFuturePhpVersionCheck),
        Box::new(NoDeadPhpVersionCheck),
        Box::new(NoUnusedPublicMethodParam),
        Box::new(NoUnusedPrivateMethodParam),
        Box::new(NoUnusedPrivateProperty),
        Box::new(NoRedundantPropertyDefault),
        Box::new(PreferPropertyDefault),
        Box::new(PreferInArrayOverForeach),
        Box::new(PreferCoalesceChain),
        Box::new(PreferCoalesceOverForeach),
        Box::new(PreferPropertyOverLocalGetter),
        Box::new(NoEmptyCatch),
        Box::new(NoUnsafeFinally),
        Box::new(NoAssignInCondition),
        Box::new(NoUnusedLabel),
        Box::new(NoRedundantWriteVisibility),
        Box::new(NoIteratorToArrayInForeach),
        Box::new(NoInvalidOpenTag),
        Box::new(NoNegatedCondition),
        Box::new(NoUnusedGlobal),
        Box::new(NoUnusedStatic),
        Box::new(NoDuplicateMatchArm),
        Box::new(NoUselessCondition),
        Box::new(NoConditionExactReturn),
        Box::new(NoOverwrittenAssignment),
        Box::new(NoRedundantCondition),
        Box::new(NoEmptyLoop),
        Box::new(NoUnusedExpression),
        Box::new(NoSoleValueSprintf),
        Box::new(NoCompact),
        Box::new(RequireIsAAllowString),
        Box::new(AddCsvEscapeArgument),
        Box::new(PreferRoundModeEnum),
        Box::new(NoFinfoBufferContext),
        Box::new(NoNullDebugInfo),
        Box::new(NoOutOfRangeChr),
        Box::new(PreferOrdFirstByte),
        Box::new(PreferArrayFirstLast),
        Box::new(PreferClamp),
        Box::new(PreferArrayAll),
        Box::new(PreferArrayAny),
        Box::new(PreferArrayFind),
        Box::new(PreferArrayFindKey),
        Box::new(NoEval),
        Box::new(NoGlobal),
        Box::new(NoRequestVariable),
        Box::new(NoDynamicName),
        Box::new(NoUnderscoreClass),
        Box::new(NoVoidReferenceReturn),
        Box::new(NoNestedTernary),
        Box::new(NoEmpty),
        Box::new(NoShortTernary),
        Box::new(NoVariableVariable),
        Box::new(NoExplicitConstructorCall),
        Box::new(NoAssignInArgument),
        Box::new(NoSwappedExplodeArguments),
        Box::new(RequirePregQuoteDelimiter),
        Box::new(NoFfi),
        Box::new(NoInsecureComparison),
        Box::new(NoLiteralPassword),
        Box::new(NoHashEmoji),
        Box::new(NoMissingFormatArgument),
        Box::new(NoParameterShadowing),
        Box::new(NoUnreachableLoop),
        Box::new(PreferInstanceofStringable),
        Box::new(NoArrayAccumulationInLoop),
        Box::new(PreferEarlyContinue),
        Box::new(NoTaintedData),
        Box::new(NoProtectedConstantInFinal),
        Box::new(NoProtectedMethodInFinal),
        Box::new(NoProtectedPropertyInFinal),
        Box::new(PreferSelfOverStatic),
        Box::new(RequireStrictArraySearch),
        Box::new(NoEscapedQuotes),
        Box::new(NoMultiAssign),
        Box::new(RequireStatementPadding),
        Box::new(RequireAssignSetPadding),
        Box::new(PreferPhpVersionId),
        Box::new(NoBoolTernary),
        Box::new(NoUselessTernary),
        Box::new(NoFalseTernary),
        Box::new(PreferEmptyStringIdentical),
        Box::new(NoBooleanLiteralCompare),
        Box::new(PreferEmptyArrayIdentical),
        Box::new(PreferMinMax),
        Box::new(PreferSerializeMethod),
        Box::new(PreferUnserializeMethod),
        Box::new(NoVoidOnMagicMethod),
        Box::new(NoUselessReadonly),
        Box::new(NoCallUserFuncMethod),
        Box::new(NoMixedOverruledByNative),
        Box::new(NoRequiredAfterOptional),
        Box::new(NoNegatedComparison),
        Box::new(PreferCoalesceOverEmptyTernary),
        Box::new(NoForInitOverwrite),
        Box::new(NoLooseEquality),
        Box::new(NoDebugCalls),
        Box::new(RequireNamedArgumentOrder),
        Box::new(RequireAttributeArgumentOrder),
        Box::new(NoLoopVariableOverwrite),
        Box::new(NoUnusedPrivateConstant),
        Box::new(PreferVariadicParam),
        Box::new(PreferSpread),
        Box::new(NoStaticConstructorCall),
        Box::new(NoNullTagValue),
        Box::new(RequireStrictFunctionCall),
        Box::new(PreferCatchNameFromType),
        Box::new(NoParentCallWithoutParent),
        Box::new(NoUselessVariable::default()),
        Box::new(NoNullDefaultArgument),
        Box::new(NoNullDefaultNamedArgument),
        Box::new(NoUnusedConstructorParam),
        Box::new(NoDeadCatch),
        Box::new(AddParamTypeSprintf),
        Box::new(AddDatetimePropertyType),
        Box::new(PreferClassKeyword),
        Box::new(RequireParentVisibility),
        Box::new(NoUnusedPromotedProperty),
        Box::new(NoBooleanFlagParameter::default()),
        Box::new(NoElseClause),
        Box::new(NoEmptyFile),
        Box::new(NoExcessiveCyclomatic::default()),
        Box::new(NoExcessiveEnumCases::default()),
        Box::new(NoExcessiveHalstead::default()),
        Box::new(NoExcessiveKanDefect::default()),
        Box::new(NoExcessiveMethods::default()),
        Box::new(NoExcessiveNesting::default()),
        Box::new(NoExcessiveParameters::default()),
        Box::new(NoExcessiveProperties::default()),
        Box::new(NoGoto),
        Box::new(PreferNamedLiteralArgument::default()),
        Box::new(RequireAssertDescription),
        Box::new(NoIniSet),
        Box::new(NoSprintfConcat),
        Box::new(NoAmbiguousFunction),
        Box::new(NoAmbiguousConstant),
        Box::new(PreferCreateFromTimestamp),
        Box::new(RequireTodoReference),
        Box::new(NoInvalidPhpdoc),
        Box::new(NoInlineHtml),
        Box::new(NoIsset::default()),
        Box::new(NoMultipleClassesPerFile),
        Box::new(PreferWhileLoop),
        Box::new(RequireNamespace),
        Box::new(RequireSortedIntegerKeys),
        Box::new(NoRestrictedFunctions::default()),
        Box::new(NoRestrictedInstantiation::default()),
        Box::new(RequireInterpolationStyle::default()),
        Box::new(NoSideEffectsWithDeclarations::default()),
        Box::new(RequireNamingConvention::default()),
        Box::new(RequirePhpdoc::default()),
        Box::new(NoEmptyMethod),
        Box::new(NoUnusedPrivateMethod),
        Box::new(NoUselessConstructor),
        Box::new(NoUselessOverride),
        Box::new(NoRedundantDefaultArgument),
        Box::new(NoDynamicMethodCall),
        Box::new(NoDynamicMethodCallable),
        Box::new(NoDynamicProperty),
        Box::new(NoDynamicStaticCall),
        Box::new(NoDynamicStaticCallable),
        Box::new(NoDynamicStaticProperty),
        Box::new(PreferClassNameConstant::default()),
        Box::new(RequireStrictInArray),
        Box::new(NoExtraArguments),
        Box::new(RequireArrayFilterCallback),
        Box::new(NoConstantCondition),
        Box::new(PreferInstanceMethod),
        Box::new(NoUnusedTestMethodParam),
        Box::new(RequirePreviousException),
        Box::new(NoUselessFilterVar),
        Box::new(NoUselessMath),
        Box::new(NoUselessCast),
        Box::new(NoAlwaysTrueInstanceof),
        Box::new(NoTypedPropertyInstanceof),
        Box::new(NoRedundantForeachGuard),
        Box::new(NoAlwaysFalseCondition),
        Box::new(NoAlwaysTrueIf),
        Box::new(NoAlwaysTrueInstanceofAssert),
        Box::new(NoConstantViaVariable),
        Box::new(RequireConsistentReturn),
        Box::new(NoPhpdocClassCaseMismatch),
        Box::new(PreferPropertyExists),
        Box::new(PreferReturnedCondition),
        Box::new(NoInvalidVarTag),
        Box::new(PreferInlineArrayReturn),
        Box::new(NoUselessVarTag),
        Box::new(NoUselessParamTag),
        Box::new(NoUselessReturnTag),
        Box::new(NoInvalidReturnTag),
        Box::new(NoUselessPhpdocType),
        Box::new(NoRedundantNullableCheck),
        Box::new(NoUnusedAssignment),
    ]
}
