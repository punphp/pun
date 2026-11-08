<?php
// Every option of every configurable fixer Pint ships (PHP-CS-Fixer built-ins + Pint's custom fixers), from the
// pinned .oracle/pint: name => [{name, types, values, default, deprecated}]. `values` is null when any value of the
// types is allowed (or a closure checks it). Feeds crates/pun-config/tests/fixer_options.json, which the tests check
// mapping.toml's [options.*] tables against:
//   php scripts/export-fixer-options.php > crates/pun-config/tests/fixer_options.json
[$pint, $config] = require __DIR__.'/pint-boot.php';
$factory = (new PhpCsFixer\FixerFactory)->registerBuiltInFixers()->registerCustomFixers($config->getCustomFixers());
$out = [];
foreach (array_map(pint_unwrap(...), $factory->getFixers()) as $f) {
    if (! $f instanceof PhpCsFixer\Fixer\ConfigurableFixerInterface) {
        continue;
    }
    $opts = [];
    foreach ($f->getConfigurationDefinition()->getOptions() as $o) {
        $values = $o->getAllowedValues();
        if ($values !== null) {
            $flat = [];
            foreach ($values as $v) {
                if ($v instanceof Closure) {
                    $flat = null;
                    break;
                }
                if ($v instanceof PhpCsFixer\FixerConfiguration\AllowedValueSubset) {
                    $flat = array_merge($flat, $v->getAllowedValues());
                } else {
                    $flat[] = $v;
                }
            }
            $values = $flat;
        }
        $opts[] = [
            'name' => $o->getName(),
            'types' => $o->getAllowedTypes(),
            'values' => $values,
            'default' => $o->hasDefault() ? $o->getDefault() : null,
            'deprecated' => $o instanceof PhpCsFixer\FixerConfiguration\DeprecatedFixerOptionInterface,
        ];
    }
    $out[$f->getName()] = $opts;
}
ksort($out);
echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n";
