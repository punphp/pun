<?php

// PHPStan 2.3.1; phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php capture.php /absolute/path/to/phpstan.phar > expected.json
declare(strict_types=1);

$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-builtin-purity-override-' . getmypid();
mkdir($root);
try {
    $files = ["$root/contracts.php"];
    copy(__DIR__ . '/contracts.php', $files[0]);
    $out = [];
    foreach (['enabled', 'disabled', 'no-overrides'] as $mode) {
        $config = "parameters:\n    phpVersion: 80500\n    featureToggles:\n"
            . '        reportMethodPurityOverride: ' . ($mode === 'disabled' ? 'false' : 'true') . "\n";
        if ($mode === 'no-overrides') {
            $config .= "    customRulesetUsed: true\nautowiredAttributeServices:\n    level: null\nservices:\n";
            foreach (['CallMethodsRule', 'ConsistentConstructorRule', 'MethodCallWithPossiblyRenamedNamedArgumentRule'] as $rule) {
                $config .= "    -\n        class: PHPStan\\Rules\\Methods\\$rule\n        tags:\n            - phpstan.rules.rule\n";
            }
        }
        file_put_contents("$root/$mode.neon", $config);
        $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
            "$root/cache-$mode", ['phar://' . $phar . '/conf/config.level3.neon', "$root/$mode.neon"], $files,
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles($files);
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
        if ($result->getInternalErrors() !== []) {
            throw new RuntimeException('PHPStan internal errors');
        }
        $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)->finalize($result, onlyFiles: true, debug: true);
        $errors = [];
        foreach (array_merge($result->getErrors(), $result->getCollectorErrors()) as $error) {
            $errors[] = [
                'file' => basename($error->getFile()), 'line' => $error->getLine(),
                'identifier' => $error->getIdentifier(), 'message' => $error->getMessage(),
                'tip' => $error->getTip(), 'ignorable' => $error->canBeIgnored(),
            ];
        }
        usort($errors, static fn($a, $b) => [$a['file'], $a['line'], $a['message']] <=> [$b['file'], $b['line'], $b['message']]);
        $out[$mode] = $errors;
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
