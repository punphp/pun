# Fixer porting split

Tick `[x]` when the fixer passes 100% of its upstream fixture cases and `pun-harness format --fixer <name>` shows no corpus diffs.
Skipped fixture cases must be listed next to the entry with the reason.

Analyzer rule: the owner ports it. If you need an analyzer its owner hasn't committed yet, port other fixers first;
if truly blocked, add `CLAIMED by agent N` next to the analyzer under its owner below, commit that line, then port it yourself.
TokensAnalyzer lives in pun-tokens (already committed) — extend it there with small additive edits only.

## Agent 1: whitespace, blank lines, spacing

Analyzers owned: WhitespacesAnalyzer, WhitespacesFixerConfig, Preg (pcre2 wrapper with PHP preg_* semantics — do first, everyone needs it), Utils, CommentsAnalyzer, AlternativeSyntaxAnalyzer, BlocksAnalyzer.

- [x] array_indentation
- [x] binary_operator_spaces
- [x] blank_line_after_namespace
- [x] blank_line_after_opening_tag
- [x] blank_line_before_statement
- [x] blank_line_between_import_groups
- [x] blank_lines_before_namespace
- [x] braces_position
- [x] cast_spaces
- [x] clean_namespace
- [x] concat_space
- [x] control_structure_braces
- [x] encoding
- [x] full_opening_tag
- [x] general_phpdoc_tag_rename
- [x] global_namespace_import
- [x] indentation_type
- [x] line_ending
- [x] linebreak_after_opening_tag
- [x] method_argument_space
- [x] method_chaining_indentation
- [x] multiline_whitespace_before_semicolons
- [x] no_blank_lines_after_class_opening
- [x] no_blank_lines_after_phpdoc
- [x] no_closing_tag (6 `testWithShortOpenTag` cases are skipped upstream too: need `short_open_tag=On`)
- [x] no_empty_statement (1 `testWithShortOpenTag` case is skipped upstream too: needs `short_open_tag=On`)
- [x] no_extra_blank_lines
- [x] no_leading_namespace_whitespace
- [x] no_multiline_whitespace_around_double_arrow
- [x] no_multiple_statements_per_line
- [x] no_singleline_whitespace_before_semicolons
- [x] no_space_around_double_colon
- [x] no_spaces_after_function_name
- [x] no_spaces_around_offset

## Agent 2: syntax, casing, control flow, operators, strictness

Analyzers owned: FunctionsAnalyzer, ArgumentsAnalyzer, SwitchAnalyzer, ControlCaseStructuresAnalyzer, GotoLabelAnalyzer, ReferenceAnalyzer, AbstractNoUselessElseFixer, AbstractFunctionReferenceFixer.

- [ ] no_trailing_comma_in_singleline
- [x] no_trailing_whitespace
- [x] no_trailing_whitespace_in_comment
- [x] no_unneeded_braces
- [x] no_whitespace_before_comma_in_array
- [x] no_whitespace_in_blank_line
- [x] not_operator_with_successor_space
- [x] object_operator_without_whitespace
- [x] phpdoc_indent
- [x] phpdoc_inline_tag_normalizer
- [x] phpdoc_single_line_var_spacing
- [x] phpdoc_tag_type
- [x] single_blank_line_at_eof
- [ ] single_class_element_per_statement
- [ ] single_import_per_statement
- [ ] single_line_after_imports
- [x] single_line_comment_spacing
- [x] single_line_comment_style
- [x] single_line_empty_body
- [ ] single_space_around_construct
- [x] space_after_semicolon
- [x] spaces_inside_parentheses
- [ ] statement_indentation
- [x] switch_case_semicolon_to_colon
- [x] switch_case_space
- [x] ternary_operator_spaces
- [ ] trailing_comma_in_multiline
- [x] trim_array_spaces
- [ ] type_declaration_spaces
- [x] types_spaces
- [x] unary_operator_spaces
- [x] whitespace_after_comma_in_array
- [x] Pint/laravel_blade
- [ ] array_push
- [x] array_syntax
- [x] backtick_to_shell_exec
- [x] compact_nullable_type_declaration
- [x] control_structure_continuation_position
- [ ] date_time_immutable
- [x] declare_equal_normalize
- [x] declare_parentheses
- [x] declare_strict_types

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
- [x] no_leading_import_slash
- [ ] no_unneeded_import_alias
- [ ] no_unused_imports
- [ ] no_useless_else
- [x] no_useless_return
- [ ] ordered_class_elements
- [ ] ordered_imports
- [ ] ordered_interfaces
- [ ] ordered_traits
- [ ] php_unit_set_up_tear_down_visibility
- [ ] protected_to_private
- [ ] self_accessor
- [ ] self_static_accessor
- [x] visibility_required — deprecated upstream; not ported. pun-format runs it as a proxy of modifier_keywords (like AbstractProxyFixer).
- [x] heredoc_to_nowdoc
- [x] no_empty_phpdoc
- [x] phpdoc_align
- [x] phpdoc_no_access
- [x] phpdoc_no_package
- [x] phpdoc_no_useless_inheritdoc
- [x] phpdoc_order
- [x] phpdoc_scalar
- [x] phpdoc_separation
- [x] phpdoc_trim
- [x] phpdoc_types
- [x] phpdoc_var_without_name
- [ ] no_superfluous_phpdoc_tags


## Deprecated rules (not supported)

PHP-CS-Fixer 3.95 marks 18 rules deprecated (`DeprecatedFixerInterface`, see https://cs.symfony.com/doc/rules/index.html).
Pun does not port them: enabling one fails with a message naming its successor (`pun_format::DEPRECATED`).
The one exception is `visibility_required`, which Pint configs such as Pinkary's still enable; upstream it is a pure
proxy of `modifier_keywords`, so Pun runs a second `modifier_keywords` under that name.
