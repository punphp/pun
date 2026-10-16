<?php
// name => {priority, risky, configurable} for every fixer Pint ships (PHP-CS-Fixer built-ins + Pint's custom fixers),
// from the pinned .oracle/pint. Feeds crates/pun-fixers/tests/metadata.json (checked by tests/cases.rs):
//   php scripts/fixer-metadata.php > crates/pun-fixers/tests/metadata.json
[$pint, $config] = require __DIR__.'/pint-boot.php';
$factory = (new PhpCsFixer\FixerFactory)->registerBuiltInFixers()->registerCustomFixers($config->getCustomFixers());
$out = [];
foreach ($factory->getFixers() as $f) {
    $out[$f->getName()] = [
        'priority' => $f->getPriority(),
        'risky' => $f->isRisky(),
        'configurable' => $f instanceof PhpCsFixer\Fixer\ConfigurableFixerInterface,
    ];
}
ksort($out);
echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES)."\n";
