<?php

// Boot the pinned Pint source (.oracle/pint) for pint.json at $argv[1] (or none = laravel
// preset) and return [$pintDir, $config, $fixerFactory, $preset]. $fixerFactory holds the
// configured fixers exactly as Pint's ConfigurationResolver would build them.

$pint = dirname(__DIR__).'/.oracle/pint';
$autoloader = require $pint.'/vendor/autoload.php';
$autoloader->addClassMap([
    'PhpCsFixer\FixerFactory' => $pint.'/overrides/FixerFactory.php',
    'PhpCsFixer\Runner\Parallel\ProcessFactory' => $pint.'/overrides/Runner/Parallel/ProcessFactory.php',
]);
$app = require $pint.'/bootstrap/app.php';
$app->make(Illuminate\Contracts\Console\Kernel::class)->bootstrap();

$repo = new App\Repositories\ConfigurationJsonRepository($argv[1] ?? null, null);
$app->instance(App\Repositories\ConfigurationJsonRepository::class, $repo);

$preset = $repo->preset();
$config = require $pint.'/resources/presets/'.$preset.'.php';

$ruleSet = new PhpCsFixer\RuleSet\RuleSet($config->getRules());
$factory = (new PhpCsFixer\FixerFactory)
    ->registerBuiltInFixers()
    ->registerCustomFixers($config->getCustomFixers())
    ->useRuleSet($ruleSet)
    ->setWhitespacesConfig(new PhpCsFixer\WhitespacesFixerConfig($config->getIndent(), $config->getLineEnding()));

return [$pint, $config, $factory, $preset];
