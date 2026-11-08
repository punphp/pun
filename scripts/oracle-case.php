<?php
// Oracle for hand-written fixer cases: runs the real PHP-CS-Fixer fixer (exactly like AbstractFixerTestCase::doTest's fix
// step) and appends {fixer, config?, input, expected} to crates/pun-fixers/tests/cases/<fixer>.json (run by tests/cases.rs).
// usage: php scripts/oracle-case.php <fixer> <input-file> [config-json]   Prints the fixed code.
//        php scripts/oracle-case.php --regenerate   Re-runs every case in tests/cases/ (after a PHP-CS-Fixer bump): the
//        expected output of each fix case, the message of each configuration error case. Left alone: the
//        Pint/phpdoc_type_annotations_only cases (scripts/export-pint-fixer-cases.php) and cases with a `source` (a pint run).
require __DIR__.'/../.oracle/php-cs-fixer/vendor/autoload.php';

use PhpCsFixer\FixerFactory;
use PhpCsFixer\Tokenizer\Tokens;
use PhpCsFixer\WhitespacesFixerConfig;

require __DIR__.'/../.oracle/pint/app/Fixers/TypeAnnotationsOnlyFixer.php';

function oracle_fixer(string $name): PhpCsFixer\Fixer\FixerInterface
{
    $factory = (new FixerFactory())->registerBuiltInFixers()->registerCustomFixers([new App\Fixers\TypeAnnotationsOnlyFixer()]);
    foreach ($factory->getFixers() as $f) {
        if ($f->getName() === $name) {
            return $f;
        }
    }
    fwrite(STDERR, "unknown fixer $name\n");
    exit(1);
}

function oracle_fix(string $name, ?array $config, string $input, string $path): string
{
    $fixer = oracle_fixer($name);
    if ($fixer instanceof PhpCsFixer\Fixer\ConfigurableFixerInterface) {
        $fixer->configure($config ?? []);
    }
    if ($fixer instanceof PhpCsFixer\Fixer\WhitespacesAwareFixerInterface) {
        $fixer->setWhitespacesConfig(new WhitespacesFixerConfig());
    }
    Tokens::clearCache();
    $tokens = Tokens::fromCode($input);
    $fixer->fix(new SplFileInfo($path), $tokens);

    return $tokens->generateCode();
}

if ('--regenerate' === ($argv[1] ?? null)) {
    foreach (glob(__DIR__.'/../crates/pun-fixers/tests/cases/*.json') as $out) {
        // Decode as objects so `{}` configs survive the round trip.
        $cases = json_decode(file_get_contents($out), false);
        $changed = 0;
        foreach ($cases as $case) {
            if ('Pint/phpdoc_type_annotations_only' === $case->fixer || isset($case->source)) {
                continue;
            }
            $config = isset($case->config) ? json_decode(json_encode($case->config, JSON_PRESERVE_ZERO_FRACTION), true) : null;
            if (isset($case->error)) {
                try {
                    oracle_fixer($case->fixer)->configure($config);
                    $error = '(configuration is valid)';
                } catch (Throwable $e) {
                    $error = $e->getMessage();
                }
                $changed += $error !== $case->error;
                $case->error = $error;
                continue;
            }
            $expected = oracle_fix($case->fixer, $config, $case->input, $case->path ?? __FILE__);
            $changed += $expected !== $case->expected;
            $case->expected = $expected;
        }
        if ($changed > 0) {
            echo basename($out), ": $changed changed\n";
            file_put_contents($out, json_encode($cases, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE | JSON_PRESERVE_ZERO_FRACTION)."\n");
        }
    }
    exit;
}

[$_, $name, $file] = $argv;
$config = isset($argv[3]) && '' !== $argv[3] ? json_decode($argv[3], true, 512, JSON_THROW_ON_ERROR) : null;
$input = file_get_contents($file);
$expected = oracle_fix($name, $config, $input, __FILE__);
echo $expected;

$out = __DIR__.'/../crates/pun-fixers/tests/cases/'.str_replace('/', '_', $name).'.json';
$cases = is_file($out) ? json_decode(file_get_contents($out), true) : [];
$case = ['fixer' => $name] + (null !== $config ? ['config' => $config] : []) + ['input' => $input, 'expected' => $expected];
if (!in_array($case, $cases, false)) {
    $cases[] = $case;
    file_put_contents($out, json_encode($cases, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n");
}
