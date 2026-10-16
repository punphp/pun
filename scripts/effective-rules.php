<?php

// Dump the effective, fully-resolved PHP-CS-Fixer rule list Pint would run.
//
//   php scripts/effective-rules.php [path/to/pint.json]   (no arg = laravel preset, no overrides)
//
// Boots the pinned Pint source in .oracle/pint, loads its preset exactly like
// ConfigurationResolverFactory does, then resolves rules through PHP-CS-Fixer's
// RuleSet + (Pint's overridden) FixerFactory. Fixers are listed in run order.

[$pint, $config, $factory, $preset] = require __DIR__.'/pint-boot.php';
$configPath = $argv[1] ?? null;

$src = realpath($pint.'/vendor/friendsofphp/php-cs-fixer/src').'/';
$fixers = [];
foreach ($factory->getFixers() as $fixer) {
    $file = (new ReflectionClass($fixer))->getFileName();
    $fixers[] = [
        'name' => $fixer->getName(),
        'priority' => $fixer->getPriority(),
        'risky' => $fixer->isRisky(),
        'options' => $fixer instanceof PhpCsFixer\Fixer\ConfigurableFixerInterface
            ? (fn () => $this->configuration ?? null)->call($fixer)
            : null,
        'class' => get_class($fixer),
        'file' => str_starts_with($file, $src) ? 'php-cs-fixer/src/'.substr($file, strlen($src)) : 'pint/'.substr($file, strlen(realpath($pint)) + 1),
    ];
}

echo json_encode([
    'pint' => trim(shell_exec('git -C '.escapeshellarg($pint).' rev-parse HEAD')),
    'php_cs_fixer' => PhpCsFixer\Console\Application::VERSION,
    'config' => $configPath ? basename(dirname(realpath($configPath))).'/'.basename($configPath) : null,
    'preset' => $preset,
    'whitespace' => ['indent' => $config->getIndent(), 'line_ending' => $config->getLineEnding()],
    'rules_as_configured' => $config->getRules(),
    'custom_fixers' => array_map(fn ($f) => ['name' => $f->getName(), 'class' => get_class($f)], $config->getCustomFixers()),
    'fixer_count' => count($fixers),
    'fixers' => $fixers,
], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES)."\n";
