<?php

declare(strict_types=1);

/*
 * Which PHPStan parameters each rule reads, as PHPStan 2.3.5 (and the Larastan, strict-rules and deprecation-rules
 * versions in .oracle/phpstan) wire them, for `pun migrate`'s phpstan.neon parameters (context/plan/migrate.md Build 5).
 *
 *   php scripts/export-phpstan-parameters.php <rule classes JSON>   prints {"rules": {class: [parameter]},
 *                                                                    "parameters": [parameter], "levels": [parameter]}
 *
 * A rule reads a parameter when its constructor, or the constructor of a PHPStan or Larastan service it takes (followed through
 * concrete classes), has `#[AutowiredParameter]` for it (the `ref`, else the argument's name), or when a service
 * definition in PHPStan's conf/*.neon or an extension's neon passes `%parameter%` as one of its arguments.
 * `parameters` are the names conf/config.neon and the extensions' neon files declare (an autowired argument named
 * otherwise is a service); `levels` are those conf/config.level*.neon set, which the level decides.
 * scripts/gen-phpstan-parameters.py turns this into rules/mapping.toml's [upstream.phpstan-parameters].
 */

use PHPStan\DependencyInjection\AutowiredParameter;
use PHPStan\DependencyInjection\NeonAdapter;

$root = dirname(__DIR__);
Phar::loadPhar($root.'/.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar', 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';
require $root.'/.oracle/phpstan/vendor/autoload.php';

$neons = [
    'phar://phpstan.phar/conf/config.neon',
    'phar://phpstan.phar/conf/services.neon',
];
// glob() doesn't read phar:// paths.
foreach (scandir('phar://phpstan.phar/conf') as $f) {
    if (str_starts_with($f, 'config.level')) {
        $neons[] = 'phar://phpstan.phar/conf/'.$f;
    }
}
foreach (['larastan/larastan/extension.neon', 'phpstan/phpstan-strict-rules/rules.neon', 'phpstan/phpstan-deprecation-rules/rules.neon'] as $f) {
    $neons[] = $root.'/.oracle/phpstan/vendor/'.$f;
}

/** The `%name%` parameters a neon value refers to. */
function refs(mixed $v): array
{
    if (is_string($v) && preg_match('/^%([\w.]+)%$/', $v, $m)) {
        return [$m[1]];
    }
    if (is_array($v)) {
        return array_merge(...array_map('refs', array_values($v)), ...[[]]);
    }
    if (is_object($v) && property_exists($v, 'arguments')) {
        return refs($v->arguments);
    }
    return [];
}

$byNeon = [];
$declared = [];
$levels = [];
$adapter = new NeonAdapter([]);
foreach ($neons as $file) {
    $config = $adapter->load($file);
    // Nested parameters (`exceptions.check.missingCheckedExceptionInThrows`) by their dotted path.
    $flatten = function (array $a, string $prefix) use (&$flatten): array {
        $out = [];
        foreach ($a as $k => $v) {
            $out[] = $prefix.$k;
            if (is_array($v) && $v !== [] && !array_is_list($v)) {
                $out = array_merge($out, $flatten($v, $prefix.$k.'.'));
            }
        }
        return $out;
    };
    $names = $flatten($config['parameters'] ?? [], '');
    if (str_contains($file, '/config.level')) {
        $levels = array_merge($levels, $names);
    } else {
        $declared = array_merge($declared, $names);
    }
    foreach ($config['services'] ?? [] as $service) {
        $class = null;
        $arguments = [];
        if (is_array($service)) {
            $class = $service['class'] ?? $service['factory'] ?? null;
            $arguments = $service['arguments'] ?? [];
        }
        if (is_object($class) && property_exists($class, 'entity')) {
            $arguments = array_merge($arguments, $class->arguments);
            $class = $class->entity;
        }
        if (is_string($class)) {
            $byNeon[ltrim($class, '\\')] = array_merge($byNeon[ltrim($class, '\\')] ?? [], refs($arguments));
        }
    }
}

/** The parameters `$class` reads, through its constructor and the PHPStan services it takes. */
function reads(string $class, array $byNeon, array &$seen = []): array
{
    if (isset($seen[$class]) || !class_exists($class)) {
        return [];
    }
    $seen[$class] = true;
    $out = $byNeon[$class] ?? [];
    $constructor = (new ReflectionClass($class))->getConstructor();
    foreach ($constructor?->getParameters() ?? [] as $p) {
        foreach ($p->getAttributes(AutowiredParameter::class) as $a) {
            $ref = $a->getArguments()['ref'] ?? $a->getArguments()[0] ?? null; // (the PHAR passes it positionally)
            $out[] = $ref === null ? $p->getName() : trim($ref, '%');
        }
        $type = $p->getType();
        if ($type instanceof ReflectionNamedType && !$type->isBuiltin() && preg_match('/^(PHPStan|Larastan)\\\\/', $type->getName())) {
            $out = array_merge($out, reads($type->getName(), $byNeon, $seen));
        }
    }
    return $out;
}

$rules = [];
foreach (json_decode($argv[1], true, flags: JSON_THROW_ON_ERROR) as $class) {
    $seen = [];
    $read = array_values(array_unique(reads($class, $byNeon, $seen)));
    sort($read);
    if ($read !== []) {
        $rules[$class] = $read;
    }
}
$sorted = function (array $a): array {
    $a = array_values(array_unique($a));
    sort($a);
    return $a;
};
echo json_encode(
    ['rules' => $rules, 'parameters' => $sorted($declared), 'levels' => $sorted($levels)],
    JSON_UNESCAPED_SLASHES | JSON_PRETTY_PRINT | JSON_THROW_ON_ERROR,
), "\n";
