# Fixer porting split

Tick `[x]` when the fixer passes 100% of its upstream fixture cases and `pun-harness format --fixer <name>` shows no corpus diffs.
Skipped fixture cases must be listed next to the entry with the reason.

Analyzer rule: the owner ports it. If you need an analyzer its owner hasn't committed yet, port other fixers first;
if truly blocked, add `CLAIMED by agent N` next to the analyzer under its owner below, commit that line, then port it yourself.
TokensAnalyzer lives in pun-tokens (already committed) — extend it there with small additive edits only.

## Agent 1: whitespace, blank lines, spacing

Analyzers owned: WhitespacesAnalyzer, WhitespacesFixerConfig, Preg (pcre2 wrapper with PHP preg_* semantics — do first, everyone needs it), Utils, CommentsAnalyzer, AlternativeSyntaxAnalyzer, BlocksAnalyzer.

- [ ] array_indentation
- [ ] binary_operator_spaces
- [ ] blank_line_after_namespace
- [ ] blank_line_after_opening_tag
- [ ] blank_line_before_statement
- [ ] blank_line_between_import_groups
- [ ] blank_lines_before_namespace
- [ ] braces_position
- [ ] cast_spaces
- [ ] clean_namespace
- [ ] concat_space
- [ ] control_structure_braces
- [ ] encoding
- [ ] full_opening_tag
- [ ] general_phpdoc_tag_rename
- [ ] global_namespace_import
- [ ] indentation_type
- [ ] line_ending
- [ ] linebreak_after_opening_tag
- [ ] method_argument_space
- [ ] method_chaining_indentation
- [ ] multiline_whitespace_before_semicolons
- [ ] no_blank_lines_after_class_opening
- [ ] no_blank_lines_after_phpdoc
- [ ] no_closing_tag
- [ ] no_empty_statement
- [ ] no_extra_blank_lines
- [ ] no_leading_namespace_whitespace
- [ ] no_multiline_whitespace_around_double_arrow
- [ ] no_multiple_statements_per_line
- [ ] no_singleline_whitespace_before_semicolons
- [ ] no_space_around_double_colon
- [ ] no_spaces_after_function_name
- [ ] no_spaces_around_offset

## Agent 2: syntax, casing, control flow, operators, strictness

Analyzers owned: FunctionsAnalyzer, ArgumentsAnalyzer, SwitchAnalyzer, ControlCaseStructuresAnalyzer, GotoLabelAnalyzer, ReferenceAnalyzer, AbstractNoUselessElseFixer, AbstractFunctionReferenceFixer.

- [ ] no_trailing_comma_in_singleline
- [ ] no_trailing_whitespace
- [ ] no_trailing_whitespace_in_comment
- [ ] no_unneeded_braces
- [ ] no_whitespace_before_comma_in_array
- [ ] no_whitespace_in_blank_line
- [ ] not_operator_with_successor_space
- [ ] object_operator_without_whitespace
- [ ] phpdoc_indent
- [ ] phpdoc_inline_tag_normalizer
- [ ] phpdoc_single_line_var_spacing
- [ ] phpdoc_tag_type
- [ ] single_blank_line_at_eof
- [ ] single_class_element_per_statement
- [ ] single_import_per_statement
- [ ] single_line_after_imports
- [ ] single_line_comment_spacing
- [ ] single_line_comment_style
- [ ] single_line_empty_body
- [ ] single_space_around_construct
- [ ] space_after_semicolon
- [ ] spaces_inside_parentheses
- [ ] statement_indentation
- [ ] switch_case_semicolon_to_colon
- [ ] switch_case_space
- [ ] ternary_operator_spaces
- [ ] trailing_comma_in_multiline
- [ ] trim_array_spaces
- [ ] type_declaration_spaces
- [ ] types_spaces
- [ ] unary_operator_spaces
- [ ] whitespace_after_comma_in_array
- [ ] Pint/laravel_blade
- [ ] array_push
- [ ] array_syntax
- [ ] backtick_to_shell_exec
- [ ] compact_nullable_type_declaration
- [ ] control_structure_continuation_position
- [ ] date_time_immutable
- [ ] declare_equal_normalize
- [ ] declare_parentheses
- [ ] declare_strict_types

## Agent 3: imports, namespaces, classes

Analyzers owned: NamespaceUsesAnalyzer, NamespacesAnalyzer, ClassyAnalyzer, AttributeAnalyzer, Analysis/* structs.

- [x] elseif
- [x] function_declaration
- [x] include
- [x] increment_style
- [x] integer_literal_case
- [x] list_syntax
- [x] lowercase_cast
- [x] lowercase_keywords
- [x] lowercase_static_reference
- [x] magic_method_casing
- [ ] mb_str_functions
- [ ] modernize_types_casting
- [ ] native_function_casing
- [ ] native_type_declaration_casing
- [ ] no_alias_functions
- [x] no_alias_language_construct_call
- [ ] no_alternative_syntax
- [x] no_binary_string
- [x] no_mixed_echo_print
- [x] no_short_bool_cast
- [ ] no_superfluous_elseif
- [ ] no_unneeded_control_parentheses
- [ ] no_unreachable_default_argument_value
- [x] no_unset_cast
- [x] normalize_index_brace
- [ ] nullable_type_declaration
- [ ] nullable_type_declaration_for_default_null_value
- [ ] php_unit_method_casing
- [x] return_type_declaration
- [x] short_scalar_cast
- [x] single_quote
- [x] standardize_not_equals
- [x] strict_comparison
- [ ] yoda_style
- [ ] class_attributes_separation
- [ ] class_definition
- [x] constant_case
- [ ] final_class
- [ ] final_internal_class

## Agent 4: class modifiers, phpdoc, comments

Analyzers owned: DocBlock (DocBlock, Line, Annotation, Tag, TagComparator, TypeExpression), AbstractPhpdocTypesFixer, AbstractProxyFixer.

- [ ] final_public_method_for_abstract_class
- [ ] fully_qualified_strict_types
- [ ] lambda_not_used_import
- [x] magic_constant_casing
- [ ] modifier_keywords
- [ ] no_leading_import_slash
- [ ] no_unneeded_import_alias
- [ ] no_unused_imports
- [ ] no_useless_else
- [ ] no_useless_return
- [ ] ordered_class_elements
- [ ] ordered_imports
- [ ] ordered_interfaces
- [ ] ordered_traits
- [ ] php_unit_set_up_tear_down_visibility
- [ ] protected_to_private
- [ ] self_accessor
- [ ] self_static_accessor
- [ ] visibility_required
- [ ] heredoc_to_nowdoc
- [ ] no_empty_phpdoc
- [ ] phpdoc_align
- [ ] phpdoc_no_access
- [ ] phpdoc_no_package
- [ ] phpdoc_no_useless_inheritdoc
- [ ] phpdoc_order
- [ ] phpdoc_scalar
- [ ] phpdoc_separation
- [ ] phpdoc_trim
- [ ] phpdoc_types
- [ ] phpdoc_var_without_name
- [ ] no_superfluous_phpdoc_tags

