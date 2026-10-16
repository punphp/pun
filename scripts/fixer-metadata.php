<?php
// name => {priority, risky, configurable} for every fixer Pint ships (PHP-CS-Fixer built-ins + Pint's custom fixers),
// from the pinned .oracle/pint. Feeds crates/pun-fixers/tests/metadata.json (checked by tests/cases.rs):
//   php scripts/fixer-metadata.php > crates/pun-fixers/tests/metadata.json
// With --gitlab: name => [summary, docs path] as GitlabReporter reads them (its own FixerFactory, built-ins only):
//   php scripts/fixer-metadata.php --gitlab > crates/pun-cli/src/gitlab_fixers.json
[$pint, $config] = require __DIR__.'/pint-boot.php';
$out = [];
if (in_array('--gitlab', $argv, true)) {
    $locator = new PhpCsFixer\Documentation\DocumentationLocator;
    foreach ((new PhpCsFixer\FixerFactory)->registerBuiltInFixers()->getFixers() as $f) {
        $out[$f->getName()] = [$f->getDefinition()->getSummary(), substr($locator->getFixerDocumentationFileRelativePath($f), 0, -4)];
    }
    ksort($out);
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n";
    exit;
}
$factory = (new PhpCsFixer\FixerFactory)->registerBuiltInFixers()->registerCustomFixers($config->getCustomFixers());
foreach ($factory->getFixers() as $f) {
    $out[$f->getName()] = [
        'priority' => $f->getPriority(),
        'risky' => $f->isRisky(),
        'configurable' => $f instanceof PhpCsFixer\Fixer\ConfigurableFixerInterface,
    ];
}
ksort($out);
echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES)."\n";
