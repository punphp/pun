<?php
// Oracle for invalid fixer configurations: runs the real PHP-CS-Fixer fixer's configure() and appends
// {fixer, config, error} to crates/pun-fixers/tests/cases/<fixer>.json (run by tests/cases.rs: configure must fail with
// exactly that message). usage: php scripts/oracle-config-error.php <fixer> <config-json>   Prints the message.
require __DIR__.'/../.oracle/php-cs-fixer/vendor/autoload.php';

[$_, $name, $json] = $argv;
$config = json_decode($json, true, 512, JSON_THROW_ON_ERROR);
$fixer = null;
foreach ((new PhpCsFixer\FixerFactory())->registerBuiltInFixers()->getFixers() as $f) {
    if ($f->getName() === $name) {
        $fixer = $f;
    }
}
if (!$fixer instanceof PhpCsFixer\Fixer\ConfigurableFixerInterface) {
    fwrite(STDERR, "unknown or not configurable fixer $name\n");
    exit(1);
}
try {
    $fixer->configure($config);
    fwrite(STDERR, "configuration is valid\n");
    exit(1);
} catch (PhpCsFixer\ConfigurationException\InvalidFixerConfigurationException $e) {
    $error = $e->getMessage();
}
echo $error, "\n";

$out = __DIR__."/../crates/pun-fixers/tests/cases/$name.json";
// Decode as objects so `{}` configs survive the round trip.
$cases = is_file($out) ? json_decode(file_get_contents($out), false) : [];
$case = (object) ['fixer' => $name, 'config' => json_decode($json, false), 'error' => $error];
if (!in_array(json_encode($case), array_map('json_encode', $cases), true)) {
    $cases[] = $case;
    file_put_contents($out, json_encode($cases, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n");
}
