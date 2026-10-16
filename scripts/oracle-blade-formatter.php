<?php
// Oracle for Pint/laravel_blade's formatter steps: runs one of Pint's App\PrettierFormatters classes on an input
// (preFormat for EmbeddedBladeMasker, else postFormat; PhpBlockFormatting formats PHP with the laravel preset) and
// appends {formatter, input, expected} to crates/pun-fixers/tests/blade_formatters.json (run by the fixer's unit
// tests). usage: php scripts/oracle-blade-formatter.php <ClassName> <input-file> [post-input-file]   Prints the output.
// For EmbeddedBladeMasker, post-input (what prettier made of the masked input) is restored by the same instance's
// postFormat into `expected_post`.
$root = __DIR__.'/..';
$preset = json_decode(file_get_contents("$root/docs/presets/laravel.json"), true)['rules_as_configured'];
// PhpFragmentFormatter takes the run's rules from a parallel worker's `--rules` argument when present.
$_SERVER['argv'] = ['pint', '--rules='.json_encode($preset)];
require "$root/.oracle/pint/vendor/autoload.php";

[$_, $class, $file] = $argv;
$input = file_get_contents($file);
$fqcn = "App\\PrettierFormatters\\$class";
$formatter = 'PhpBlockFormatting' === $class ? new $fqcn(new App\Support\PhpFragmentFormatter()) : new $fqcn();
$expected = 'EmbeddedBladeMasker' === $class ? $formatter->preFormat($input) : $formatter->postFormat($input);
echo $expected;
$case = ['formatter' => $class, 'input' => $input, 'expected' => $expected];
if (isset($argv[3])) {
    $case['post_input'] = file_get_contents($argv[3]);
    $case['expected_post'] = $formatter->postFormat($case['post_input']);
    echo "\n--- post:\n", $case['expected_post'];
}

$out = "$root/crates/pun-fixers/tests/blade_formatters.json";
$cases = is_file($out) ? json_decode(file_get_contents($out), true) : [];
if (!in_array($case, $cases, true)) {
    $cases[] = $case;
    file_put_contents($out, json_encode($cases, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n");
}
