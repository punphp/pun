<?php

// PHP-CS-Fixer config that formats *against* the laravel preset, used by perturb.php to
// build a "dirty" corpus variant (so every laravel/Pinkary fixer has work to undo).

return (new PhpCsFixer\Config)
    ->setRiskyAllowed(true)
    ->setUsingCache(false)
    ->setUnsupportedPhpVersionAllowed(true)
    ->setFinder(PhpCsFixer\Finder::create()->in(getenv('PERTURB_DIR'))->notName('*.blade.php')
        ->exclude(['node_modules', 'storage', 'bootstrap/cache', 'build']))
    ->setRules([
        'array_syntax' => ['syntax' => 'long'],
        'list_syntax' => ['syntax' => 'long'],
        'concat_space' => ['spacing' => 'one'],
        'binary_operator_spaces' => ['default' => 'align_single_space'],
        'braces_position' => ['functions_opening_brace' => 'same_line', 'classes_opening_brace' => 'same_line', 'control_structures_opening_brace' => 'next_line_unless_newline_at_signature_end'],
        'increment_style' => ['style' => 'pre'],
        'yoda_style' => true,
        'phpdoc_align' => ['align' => 'vertical'],
        'phpdoc_order' => ['order' => ['throws', 'return', 'param']],
        'ordered_imports' => ['sort_algorithm' => 'length'],
        'ordered_interfaces' => ['direction' => 'descend'],
        'ordered_class_elements' => ['sort_algorithm' => 'alpha'],
        'return_type_declaration' => ['space_before' => 'one'],
        'cast_spaces' => ['space' => 'none'],
        'class_attributes_separation' => ['elements' => ['const' => 'none', 'method' => 'none', 'property' => 'none']],
        'global_namespace_import' => ['import_classes' => false, 'import_constants' => false, 'import_functions' => false],
        'nullable_type_declaration' => ['syntax' => 'union'],
        'native_function_invocation' => ['include' => ['@all']],
        'no_mixed_echo_print' => ['use' => 'print'],
        'echo_tag_syntax' => ['format' => 'long'],
        'explicit_string_variable' => true,
        'phpdoc_scalar' => false,
        'blank_line_before_statement' => ['statements' => []],
        'method_argument_space' => ['on_multiline' => 'ensure_fully_multiline'],
        'trailing_comma_in_multiline' => ['elements' => ['arguments', 'parameters', 'match']],
        'operator_linebreak' => ['position' => 'end'],
        'single_line_throw' => true,
        'phpdoc_var_annotation_correct_order' => true,
        'self_accessor' => false,
        'phpdoc_types_order' => ['null_adjustment' => 'always_first'],
        'type_declaration_spaces' => false,
        'heredoc_indentation' => ['indentation' => 'same_as_start'],
        'multiline_whitespace_before_semicolons' => ['strategy' => 'new_line_for_chained_calls'],
    ]);
