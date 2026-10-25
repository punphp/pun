<?php

// php capture-archives.php /absolute/path/to/phpstan.phar > expected-phpstan-archives.json
// PHPStan 2.3.1's CLI cache postprocessing normalizes phar member traversal out of its archive and crashes
// reading the resulting dependency ("ok.php"). Invoke its real public Analyser without ResultCacheManager;
// this still runs the unmodified level-0 rule registry and the real PHP stream stat handlers.
$phar = $argv[1];
Phar::loadPhar($phar, 'phpstan-include-oracle.phar');
require 'phar://phpstan-include-oracle.phar/vendor/autoload.php';
chdir(__DIR__);
set_include_path('.:/usr/share/php:' . __DIR__ . '/archives');
$container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
    sys_get_temp_dir() . '/pun-include-archives-oracle',
    ['phar://phpstan-include-oracle.phar/conf/config.level0.neon', __DIR__ . '/phpstan-archives.neon'],
    [__DIR__ . '/archives.php'],
);
$result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([__DIR__ . '/archives.php'], debug: true);
$errors = [];
foreach ($result->getErrors() as $error) {
    if (!str_ends_with($error->getIdentifier() ?? '', '.fileNotFound')) {
        throw new RuntimeException($error->getMessage());
    }
    $errors[] = [
        'file' => str_replace(__DIR__, '$ROOT', $error->getFile()),
        'line' => $error->getLine(),
        'identifier' => $error->getIdentifier(),
        'message' => str_replace(__DIR__, '$ROOT', $error->getMessage()),
        'tip' => $error->getTip(),
        'ignorable' => $error->canBeIgnored(),
    ];
}
echo json_encode($errors, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
