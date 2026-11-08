<?php
// Oracle for Pint/laravel_blade's formatter steps: runs one of Pint's App\PrettierFormatters classes on an input
// (preFormat for the pre-formatters EmbeddedBladeMasker, AlpineMaskPatterns and EscapedDirectiveSpacing, else
// postFormat; PhpBlockFormatting formats PHP with the laravel preset) and appends {formatter, input, expected} to
// crates/pun-fixers/tests/blade_formatters.json (run by the fixer's unit tests).
// usage: php scripts/oracle-blade-formatter.php <ClassName> <input-file> [post-input-file]   Prints the output.
//        php scripts/oracle-blade-formatter.php --regenerate   Re-runs every case in the JSON (after a Pint bump);
//                                                             cases of a class Pint no longer has are dropped.
// For a pre-formatter, post-input (what prettier made of the masked input) is restored by the same instance's
// postFormat into `expected_post` (null when it throws UnrestorableContentException).
$root = __DIR__.'/..';
// PRESET=<name> (default laravel): the preset whose getRules() PhpBlockFormatting runs (pun-config's exports).
$presetFor = function (string $presetName) use ($root): array {
    return 'laravel' === $presetName
        ? json_decode(file_get_contents("$root/crates/pun-config/presets/laravel.json"), true)['rules_as_configured']
        : json_decode(file_get_contents("$root/crates/pun-config/presets/$presetName.json"), true);
};
require "$root/.oracle/pint/vendor/autoload.php";

$run = function (string $class, string $input, ?string $postInput, string $presetName) use ($presetFor): array {
    // PhpFragmentFormatter takes the run's rules from a parallel worker's `--rules` argument when present.
    $_SERVER['argv'] = ['pint', '--rules='.json_encode($presetFor($presetName))];
    $fqcn = "App\\PrettierFormatters\\$class";
    $formatter = 'PhpBlockFormatting' === $class ? new $fqcn(new App\Support\PhpFragmentFormatter()) : new $fqcn();
    $expected = $formatter instanceof App\Contracts\PrettierPreFormatter ? $formatter->preFormat($input) : $formatter->postFormat($input);
    $case = ['formatter' => $class, 'input' => $input, 'expected' => $expected] + ('laravel' === $presetName ? [] : ['preset' => $presetName]);
    if (null !== $postInput) {
        $case['post_input'] = $postInput;
        try {
            $case['expected_post'] = $formatter->postFormat($postInput);
        } catch (App\Exceptions\UnrestorableContentException) {
            $case['expected_post'] = null;
        }
    }

    return $case;
};

$out = "$root/crates/pun-fixers/tests/blade_formatters.json";
$cases = is_file($out) ? json_decode(file_get_contents($out), true) : [];
$save = fn (array $cases) => file_put_contents($out, json_encode($cases, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n");

if ('--regenerate' === ($argv[1] ?? null)) {
    $new = [];
    foreach ($cases as $case) {
        if (!class_exists("App\\PrettierFormatters\\{$case['formatter']}")) {
            continue;
        }
        $new[] = $run($case['formatter'], $case['input'], $case['post_input'] ?? null, $case['preset'] ?? 'laravel');
    }
    $save($new);
    echo count($cases), ' cases -> ', count($new), "\n";
    exit;
}

[$_, $class, $file] = $argv;
$case = $run($class, file_get_contents($file), isset($argv[3]) ? file_get_contents($argv[3]) : null, getenv('PRESET') ?: 'laravel');
echo $case['expected'];
if (array_key_exists('expected_post', $case)) {
    echo "\n--- post:\n", $case['expected_post'] ?? '(UnrestorableContentException)';
}
if (!in_array($case, $cases, true)) {
    $cases[] = $case;
    $save($cases);
}
