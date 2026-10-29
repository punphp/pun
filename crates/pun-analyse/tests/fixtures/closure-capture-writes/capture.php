<?php
// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php capture.php /absolute/path/to/phpstan.phar [output.json] [case,...]
declare(strict_types=1);

$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$temporary = sys_get_temp_dir() . '/pun-closure-capture-writes-' . getmypid();
mkdir($temporary);
$out = [];
try {
    foreach (isset($argv[3]) ? explode(',', $argv[3]) : ['captures', 'overwrites', 'reads', 'boundaries', 'original-closure', 'original-constructor'] as $case) {
        $file = match ($case) {
            'original-closure' => realpath(__DIR__ . '/../rules/data/Functions/data/unused-closure-uses.php'),
            'original-constructor' => realpath(__DIR__ . '/../rules/data/Classes/data/unused-constructor-parameters.php'),
            default => __DIR__ . "/$case.php",
        };
        foreach ($case === 'captures' ? [0, 9] : [0, 1] as $level) {
            foreach ([false, true] as $precise) {
                $key = "$case-$level-" . (int) $precise;
                $config = "$temporary/$key.neon";
                file_put_contents($config, "parameters:\n    phpVersion: 80500\n    featureToggles:\n        reportPreciseLineForUnusedFunctionParameter: " . ($precise ? 'true' : 'false') . "\n");
                $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
                    "$temporary/$key", ['phar://' . $phar . "/conf/config.level$level.neon", $config], [$file],
                );
                $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
                $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
                $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
                if ($result->getInternalErrors() !== []) throw new RuntimeException('Oracle internal error');
                $errors = array_map(static fn($error) => [
                    $error->getLine(), $error->getIdentifier(), $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
                ], $result->getUnorderedErrors());
                sort($errors);
                $out[$key] = $errors;
                fwrite(STDERR, "$key: " . count($errors) . " complete records\n");
            }
        }
    }
    file_put_contents($argv[2] ?? __DIR__ . '/expected.json', json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR) . "\n");
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($temporary, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($temporary);
}
