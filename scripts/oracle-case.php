<?php
// Oracle for hand-written fixer cases: runs the real PHP-CS-Fixer fixer (exactly like AbstractFixerTestCase::doTest's fix
// step) and appends {fixer, config?, input, expected} to crates/pun-fixers/tests/cases/<fixer>.json (run by tests/cases.rs).
// usage: php scripts/oracle-case.php <fixer> <input-file> [config-json]   Prints the fixed code.
require __DIR__.'/../.oracle/php-cs-fixer/vendor/autoload.php';

use PhpCsFixer\FixerFactory;
use PhpCsFixer\Tokenizer\Tokens;
use PhpCsFixer\WhitespacesFixerConfig;

[$_, $name, $file] = $argv;
$config = isset($argv[3]) && '' !== $argv[3] ? json_decode($argv[3], true, 512, JSON_THROW_ON_ERROR) : null;
$input = file_get_contents($file);

$factory = (new FixerFactory())->registerBuiltInFixers();
$fixer = null;
foreach ($factory->getFixers() as $f) {
    if ($f->getName() === $name) {
        $fixer = $f;
    }
}
if (null === $fixer) {
    fwrite(STDERR, "unknown fixer $name\n");
    exit(1);
}
if ($fixer instanceof PhpCsFixer\Fixer\ConfigurableFixerInterface) {
    $fixer->configure($config ?? []);
}
if ($fixer instanceof PhpCsFixer\Fixer\WhitespacesAwareFixerInterface) {
    $fixer->setWhitespacesConfig(new WhitespacesFixerConfig());
}
Tokens::clearCache();
$tokens = Tokens::fromCode($input);
$fixer->fix(new SplFileInfo(__FILE__), $tokens);
$expected = $tokens->generateCode();
echo $expected;

$out = __DIR__."/../crates/pun-fixers/tests/cases/$name.json";
$cases = is_file($out) ? json_decode(file_get_contents($out), true) : [];
$case = ['fixer' => $name] + (null !== $config ? ['config' => $config] : []) + ['input' => $input, 'expected' => $expected];
if (!in_array($case, $cases, false)) {
    $cases[] = $case;
    file_put_contents($out, json_encode($cases, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n");
}
