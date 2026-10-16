<?php

declare(strict_types=1);

/*
 * Oracle for crates/pun-phpstan's config loader: resolves a PHPStan config the way `phpstan analyse` does
 * (CommandHelper::begin's config half: config discovery, project-config pre-load for level/tmpDir/paths, level file,
 * phpstan/extension-installer's GeneratedConfig, ContainerFactory::create incl. parameter validation), without
 * running bootstrapFiles, and prints {"parameters": ..., "allConfigFiles": [...]} or {"error": "<stderr text>"}
 * as JSON. Error text is what PHPStan writes to stderr, with console tags stripped.
 *
 *   php scripts/phpstan-config.php <phpstan.phar> <cwd> [-c config] [-l level] [--generated-config GeneratedConfig.php]
 *       [--generate-baseline file] [path...]
 *
 * Values that are objects are tagged: {"@class": "...", ...props}.
 */

$phar = str_starts_with($argv[1], '/') ? $argv[1] : getcwd().'/'.$argv[1]; // not realpath(): %rootDir% keeps symlinks
$cwd = realpath($argv[2]) ?: exit("no cwd\n");
$config = null;
$level = null;
$generated = null;
$generateBaseline = null;
$cliPaths = [];
for ($i = 3; $i < count($argv); $i++) {
    match ($argv[$i]) {
        '-c' => $config = $argv[++$i],
        '-l' => $level = $argv[++$i],
        '--generated-config' => $generated = $argv[++$i],
        '--generate-baseline' => $generateBaseline = $argv[++$i],
        default => $cliPaths[] = $argv[$i],
    };
}

Phar::loadPhar($phar, 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';
if (is_file($cwd.'/vendor/autoload.php')) {
    require $cwd.'/vendor/autoload.php'; // what vendor/bin/phpstan does
}
if ($generated !== null) {
    require str_starts_with($generated, '/') ? $generated : $cwd.'/'.$generated; // like the test, relative to <cwd>
}
chdir($cwd);

use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\DependencyInjection\LoaderFactory;
use PHPStan\File\FileHelper;

$err = [];
$out = static function (string $s) use (&$err): void {
    $err[] = preg_replace('~</?[a-z][^>]*>~', '', $s);
};
$fail = static function () use (&$err): never {
    echo json_encode(['error' => implode("\n", $err)], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE), "\n";
    exit(0);
};

function dumpValue(mixed $v): mixed
{
    if (is_array($v)) {
        $isList = array_is_list($v);
        $res = [];
        foreach ($v as $k => $x) {
            $res[$isList ? count($res) : (string) $k] = dumpValue($x);
        }

        return $isList ? $res : ($res === [] ? new stdClass : $res);
    }
    if (is_object($v)) {
        $res = ['@class' => preg_replace('~^_PHPStan_[0-9a-f]+\\\\~', '', get_class($v))];
        foreach ((array) $v as $k => $x) {
            $res[preg_replace('~^\0.*\0~', '', $k)] = dumpValue($x);
        }

        return $res;
    }
    if (is_float($v)) {
        return ['@float' => var_export($v, true)];
    }

    return $v;
}

try {
    $fileHelper = new FileHelper($cwd);
    $cwd = $fileHelper->getWorkingDirectory();
    if ($config === null) {
        foreach (['.phpstan.neon', 'phpstan.neon', '.phpstan.neon.dist', 'phpstan.neon.dist', '.phpstan.dist.neon', 'phpstan.dist.neon'] as $name) {
            if (is_file($cwd.DIRECTORY_SEPARATOR.$name)) {
                $config = $cwd.DIRECTORY_SEPARATOR.$name;
                break;
            }
        }
    } else {
        $config = $fileHelper->absolutizePath($config);
    }
    if ($generateBaseline !== null) {
        $generateBaseline = $fileHelper->normalizePath($fileHelper->absolutizePath($generateBaseline));
    }
    if ($config === null && $level === null) {
        $level = '0';
    }
    $containerFactory = new ContainerFactory($cwd);
    $analysedPathsFromConfig = [];
    $paths = array_map(static fn (string $p): string => $fileHelper->normalizePath($fileHelper->absolutizePath($p)), $cliPaths);
    if ($config !== null) {
        if (!is_file($config)) {
            $out(sprintf('Project config file at path %s does not exist.', $config));
            $fail();
        }
        $loader = (new LoaderFactory($fileHelper, $containerFactory->getRootDirectory(), $containerFactory->getCurrentWorkingDirectory(), $generateBaseline, ['[parameters][paths][]', '[parameters][tmpDir]']))->createLoader();
        try {
            $projectConfig = $loader->load($config, null);
        } catch (\_PHPStan_88a9c7bcc\Nette\InvalidStateException|\_PHPStan_88a9c7bcc\Nette\FileNotFoundException $e) {
            $out($e->getMessage());
            $fail();
        }
        $defaultParameters = ['rootDir' => $containerFactory->getRootDirectory(), 'currentWorkingDirectory' => $containerFactory->getCurrentWorkingDirectory(), 'env' => PHPStan\Command\Environment::getCleanedArray()];
        if (isset($projectConfig['parameters']['tmpDir'])) {
            $tmpDir = \_PHPStan_88a9c7bcc\Nette\DI\Helpers::expand($projectConfig['parameters']['tmpDir'], $defaultParameters);
        }
        if ($level === null && isset($projectConfig['parameters']['level'])) {
            $level = (string) $projectConfig['parameters']['level'];
        }
        if (isset($projectConfig['parameters']['paths'])) {
            $analysedPathsFromConfig = \_PHPStan_88a9c7bcc\Nette\DI\Helpers::expand($projectConfig['parameters']['paths'], $defaultParameters);
        }
        if (count($paths) === 0) {
            $paths = $analysedPathsFromConfig;
        }
    }
    $additional = [];
    if ($level !== null) {
        $levelConfigFile = sprintf('%s/config.level%s.neon', $containerFactory->getConfigDirectory(), $level);
        if (!is_file($levelConfigFile)) {
            $out(sprintf('Level config file %s was not found.', $levelConfigFile));
            $fail();
        }
        $additional[] = $levelConfigFile;
    }
    if (class_exists('PHPStan\ExtensionInstaller\GeneratedConfig')) {
        $dir = dirname((new ReflectionClass('PHPStan\ExtensionInstaller\GeneratedConfig'))->getFileName());
        foreach (PHPStan\ExtensionInstaller\GeneratedConfig::EXTENSIONS as $name => $ext) {
            foreach ($ext['extra']['includes'] ?? [] as $inc) {
                if (!is_string($inc)) {
                    $out(sprintf('Cannot include config from package %s, expecting string file path but got %s', $name, gettype($inc)));
                    $fail();
                }
                $path = null;
                if (isset($ext['relative_install_path'])) {
                    $path = sprintf('%s/%s/%s', $dir, $ext['relative_install_path'], $inc);
                    if (!is_file($path) || !is_readable($path)) {
                        $path = null;
                    }
                }
                $path ??= sprintf('%s/%s', $ext['install_path'], $inc);
                if (!is_file($path) || !is_readable($path)) {
                    $out(sprintf('Config file %s does not exist or isn\'t readable', $path));
                    $fail();
                }
                $additional[] = $path;
            }
        }
    }
    if ($config !== null) {
        $additional[] = $config;
    }
    $tmpDir ??= sys_get_temp_dir().'/phpstan';
    @mkdir($tmpDir, 0777, true);
    try {
        $container = $containerFactory->create($tmpDir, $additional, $paths, [], $analysedPathsFromConfig, $level ?? '0', $generateBaseline);
    } catch (\_PHPStan_88a9c7bcc\Nette\DI\InvalidConfigurationException|\_PHPStan_88a9c7bcc\Nette\Utils\AssertionException $e) {
        $out('Invalid configuration:');
        $out($e->getMessage());
        $fail();
    } catch (PHPStan\DependencyInjection\InvalidIgnoredErrorPatternsException $e) {
        $out(sprintf('Invalid %s in ignoreErrors:', count($e->getErrors()) === 1 ? 'entry' : 'entries'));
        foreach ($e->getErrors() as $error) {
            $out($error);
            $out('');
        }
        $out('To ignore non-existent paths in ignoreErrors,');
        $out('set reportUnmatchedIgnoredErrors: false in your configuration file.');
        $out('');
        $fail();
    } catch (PHPStan\DependencyInjection\InvalidExcludePathsException $e) {
        $out(sprintf('Invalid %s in excludePaths:', count($e->getErrors()) === 1 ? 'entry' : 'entries'));
        foreach ($e->getErrors() as $error) {
            $out($error);
            $out('');
        }
        $suggestOptional = $e->getSuggestOptional();
        if (count($suggestOptional) > 0) {
            $helper = $config !== null ? new PHPStan\File\ParentDirectoryRelativePathHelper(dirname($config)) : null;
            $out('If the excluded path can sometimes exist, append (?)');
            $out('to its config entry to mark it as optional. Example:');
            $out('');
            $out('parameters:');
            $out("\texcludePaths:");
            foreach ($suggestOptional as $key => $paths) {
                $out(sprintf("\t\t%s:", $key));
                foreach ($paths as $path) {
                    $out(sprintf("\t\t\t- %s (?)", $helper === null ? $path : $helper->getRelativePath($path)));
                }
            }
            $out('');
        }
        $fail();
    } catch (\_PHPStan_88a9c7bcc\Nette\Schema\ValidationException $e) {
        foreach ($e->getMessages() as $message) {
            $out('Invalid configuration:');
            $out($message);
        }
        $fail();
    } catch (PHPStan\DependencyInjection\DuplicateIncludedFilesException $e) {
        $format = count($e->getFiles()) === 1 ? "This file is included multiple times:\n- %s" : "These files are included multiple times:\n- %s";
        $out(sprintf($format, implode("\n- ", $e->getFiles())));
        $fail();
    }
    $params = $container->getParameters();
    unset($params['env']);
    echo json_encode(['parameters' => dumpValue($params)], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE | JSON_PRESERVE_ZERO_FRACTION), "\n";
} catch (Throwable $e) {
    $out($e->getMessage()); // uncaught: PHPStan's console prints it in an exception box
    $fail();
}
