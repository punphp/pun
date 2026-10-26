<?php

// php capture-numeric-admission.php /absolute/path/to/php-cs-fixer > numeric-admission.json
require $argv[1] . '/vendor/autoload.php';
$linter = new PhpCsFixer\Linter\TokenizerLinter();
$out = [];
foreach (['08', '078', '0_8', '07_9', '010000000000000000000008', '077', '07_7', '0o77', '08.0', '08e2', '01000000000000000000000'] as $literal) {
    $source = "<?php \$a=$literal;";
    try {
        $linter->lintSource($source)->check();
        $out[$literal] = null;
    } catch (PhpCsFixer\Linter\LintingException $error) {
        $out[$literal] = $error->getMessage();
    }
}
echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
