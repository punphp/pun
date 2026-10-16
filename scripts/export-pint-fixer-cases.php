<?php

// Export Pint's own Pest datasets for Pint/phpdoc_type_annotations_only into
// crates/pun-fixers/tests/cases/Pint_phpdoc_type_annotations_only.json (run by tests/cases.rs).
// Merges into the file (hand-written oracle-case.php cases are kept). Each dataset's expected value is re-derived by running Pint's fixer on the input (and must match the dataset).
//
//   php scripts/export-pint-fixer-cases.php

$pint = __DIR__.'/../.oracle/pint';
require $pint.'/vendor/autoload.php';
require $pint.'/app/Fixers/TypeAnnotationsOnlyFixer.php';

use App\Fixers\TypeAnnotationsOnlyFixer;
use PhpCsFixer\Tokenizer\Tokens;

// Minimal Pest stand-ins: record every it(...)->with([...]) dataset.
$GLOBALS['datasets'] = [];
function beforeEach(Closure $c): void {}
function it(string $name, Closure $c): object
{
    return new class($name) {
        public function __construct(private string $name) {}

        public function with(array $data): void
        {
            $GLOBALS['datasets'][$this->name] = $data;
        }
    };
}
require $pint.'/tests/Feature/Fixers/PhpdocTypeAnnotationsOnlyFixerTest.php';

// The config-directory tests only pass a path; pair it with the test's own code sample.
$configCode = "<?php\n\n// This comment should be preserved in config files\nreturn [\n    'name' => env('APP_NAME', 'Laravel'),\n];";

$out = __DIR__.'/../crates/pun-fixers/tests/cases/Pint_phpdoc_type_annotations_only.json';
$cases = is_file($out) ? json_decode(file_get_contents($out), true) : [];
foreach ($GLOBALS['datasets'] as $test => $data) {
    foreach ($data as $key => $args) {
        $path = null;
        if (str_contains($test, 'config directory')) {
            [$path] = $args;
            $input = $configCode;
            $want = null;
        } else {
            $input = $args[0];
            $want = $args[1] ?? $input;
        }
        Tokens::clearCache();
        $tokens = Tokens::fromCode($input);
        (new TypeAnnotationsOnlyFixer)->fix(new SplFileInfo($path ?? __FILE__), $tokens);
        $expected = $tokens->generateCode();
        if (null !== $want && $want !== $expected) {
            fwrite(STDERR, "dataset mismatch: $test / $key\n");
            exit(1);
        }
        $case = ['fixer' => 'Pint/phpdoc_type_annotations_only'] + (null !== $path ? ['path' => $path] : []) + ['input' => $input, 'expected' => $expected];
        if (! in_array($case, $cases, false)) {
            $cases[] = $case;
        }
    }
}
file_put_contents(
    $out,
    json_encode($cases, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n",
);
echo count($cases)." cases\n";
