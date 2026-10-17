<?php

declare(strict_types=1);

/*
 * Built-in function signatures exactly as PHPStan 2.2.7 reflects them, from the phar's own container
 * (ReflectionProvider -> NativeFunctionReflectionProvider: functionMap + PHP-version deltas, merged with
 * php-8-stubs, overlaid with stubs/*.stub PHPDocs, functionMetadata.php). Default config (no bleedingEdge,
 * so stricterFunctionMap is off).
 *
 *   php scripts/export-phpstan-symbols.php   writes crates/pun-phpstan/data/functions.json
 *                                            and crates/pun-phpstan/tests/fixtures/type-strings.json
 *
 * functions.json: {"<lowercase name>": {"name", "sideEffects": yes|no|maybe, "deprecated", "variants": [...],
 *   "named": [...]|absent}} where a variant is {"params": [{"name", "type", "optional", "variadic",
 *   "byRef": no|createsNewVariable|readsArgument, "default"?}], "return"}. A type is its toPhpDocNode()
 *   string (what Pun parses); `default` is the default value's type. One file per target PHP version
 *   would be ~99% duplicate, so entries that differ on 8.5 sit under "php85": {"<name>": entry|null}.
 *
 * type-strings.json: every type string the merged signature map (8.4 and 8.5) and the generated
 *   toPhpDocNode strings contain, with TypeStringResolver::resolve()->describe(precise()) and typeOnly().
 */

use PHPStan\Analyser\NameScope;
use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\PhpDoc\TypeStringResolver;
use PHPStan\Reflection\ParametersAcceptor;
use PHPStan\Reflection\ReflectionProvider;
use PHPStan\Reflection\SignatureMap\FunctionSignatureMapProvider;
use PHPStan\Type\Type;
use PHPStan\Type\VerbosityLevel;

$root = dirname(__DIR__);
Phar::loadPhar($root.'/.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar', 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';
require 'phar://phpstan.phar/vendor/phpstan/php-8-stubs/Php8StubsMap.php';

function container(int $phpVersion): PHPStan\DependencyInjection\Container
{
    $tmp = sys_get_temp_dir().'/pun-symbols-'.$phpVersion;
    @mkdir($tmp);
    $neon = $tmp.'/v.neon';
    file_put_contents($neon, "parameters:\n\tphpVersion: $phpVersion\n");

    return (new ContainerFactory($tmp))->create($tmp, [$neon], []);
}

/*
 * toPhpDocNode() is not faithful for a few types (it drops __benevolent, template bounds/scopes, *ERROR* and
 * *NEVER*, and turns non-printable constant strings into accessory intersections). Those are replaced by placeholders
 * before toPhpDocNode() and written back in Pun's extensions of the syntax:
 *   __benevolent<A|B>                                   (PHPStan's own syntax)
 *   __template<T, 'function f()', BOUND[, DEFAULT]>     a template parameter of that scope
 *   *ERROR*, *NEVER*                                     ErrorType, NeverType (`never` is NonAcceptingNeverType)
 *   "..."                                                constant string with control characters (PHP escapes)
 */
function t(Type $type): string
{
    $holes = [];
    $hole = function (string $text) use (&$holes): Type {
        $name = '__PunHole'.count($holes);
        $holes[$name] = $text;

        return new PHPStan\Type\ObjectType($name);
    };
    $mapped = PHPStan\Type\TypeTraverser::map($type, function (Type $t, callable $traverse) use ($hole): Type {
        if ($t instanceof PHPStan\Type\Generic\TemplateType) {
            if ($t->isArgument()) {
                throw new LogicException('template argument in a signature');
            }
            $args = [$t->getName(), var_export($t->getScope()->describe(), true), t($t->getBound())];
            if ($t->getDefault() !== null) {
                $args[] = t($t->getDefault());
            }

            return $hole('__template<'.implode(', ', $args).'>');
        }
        if ($t instanceof PHPStan\Type\BenevolentUnionType) {
            return $hole('__benevolent<'.t(new PHPStan\Type\UnionType($t->getTypes())).'>');
        }
        // Mapping members to placeholders would re-normalize (int&T becomes never), so these are written member by
        // member, in the order toPhpDocNode() uses.
        if ($t instanceof PHPStan\Type\UnionType) {
            return $hole('('.implode(' | ', array_map('t', (fn () => $this->getSortedTypes())->call($t))).')');
        }
        if ($t instanceof PHPStan\Type\IntersectionType && array_filter($t->getTypes(), fn ($m) => $m instanceof PHPStan\Type\Generic\TemplateType) !== []) {
            return $hole('('.implode(' & ', array_map('t', (fn () => $this->getSortedTypes())->call($t))).')');
        }
        if ($t instanceof PHPStan\Type\ErrorType) {
            return $hole('*ERROR*');
        }
        if ($t instanceof PHPStan\Type\NeverType && !$t instanceof PHPStan\Type\NonAcceptingNeverType) {
            return $hole('*NEVER*');
        }
        if ($t instanceof PHPStan\Type\Constant\ConstantStringType && addcslashes($t->getValue(), "\x00..\x1f") !== $t->getValue()) {
            return $hole($t->describe(VerbosityLevel::precise()));
        }

        return $traverse($t);
    });
    $s = (string) $mapped->toPhpDocNode();
    $s = preg_replace_callback('/__PunHole\d+/', fn ($m) => $holes[$m[0]], $s);
    $GLOBALS['reflected'][$GLOBALS['ctx'] ?? ''][$s][json_encode(array_map(fn ($l) => $type->describe($l), [VerbosityLevel::precise(), VerbosityLevel::value(), VerbosityLevel::typeOnly()]))] = true;

    return $s;
}

function variant(ParametersAcceptor $v): array
{
    $params = [];
    foreach ($v->getParameters() as $p) {
        $param = [
            'name' => $p->getName(),
            'type' => t($p->getType()),
            'optional' => $p->isOptional(),
            'variadic' => $p->isVariadic(),
            'byRef' => match (true) {
                $p->passedByReference()->no() => 'no',
                $p->passedByReference()->createsNewVariable() => 'createsNewVariable',
                default => 'readsArgument',
            },
        ];
        if ($p->getDefaultValue() !== null) {
            $param['default'] = t($p->getDefaultValue());
        }
        $params[] = $param;
    }

    return ['params' => $params, 'return' => t($v->getReturnType())];
}

function callable_entry(PHPStan\Reflection\FunctionReflection|PHPStan\Reflection\ExtendedMethodReflection $f): array
{
    $entry = [
        'name' => $f->getName(),
        'sideEffects' => $f->hasSideEffects()->describe(),
        'deprecated' => $f->isDeprecated()->yes(),
        'variants' => array_map('variant', $f->getVariants()),
    ];
    if ($f->getNamedArgumentsVariants() !== null) {
        $entry['named'] = array_map('variant', $f->getNamedArgumentsVariants());
    }

    return $entry;
}

/** @return array{functions: array<string, mixed>, constants: array<string, string>, classes: array<string, mixed>} */
function dump(int $phpVersion, array &$typeStrings): array
{
    $c = container($phpVersion);
    $provider = $c->getByType(ReflectionProvider::class);
    $map = $c->getByType(FunctionSignatureMapProvider::class)->getSignatureMap();
    // SignatureMapParser resolves a method's return type in its class (static, self) and parameters without one.
    foreach ($map as $key => $row) {
        $class = str_contains($key, '::') ? explode('::', $key)[0] : '';
        if ($class !== '' && $provider->hasClass($class)) {
            $class = $provider->getClass($class)->getName();
        }
        foreach (array_values($row) as $i => $s) {
            // The class only matters to self/static/parent/$this; elsewhere it would just duplicate the string.
            $typeStrings[$i === 0 && preg_match('/\\b(static|self|parent)\\b|\\$this/i', $s) ? $class : ''][$s] = true;
        }
    }
    $stubs = _PHPStan_88a9c7bcc\JetBrains\PHPStormStub\PhpStormStubsMap::class;
    $name = fn (string $n) => new PhpParser\Node\Name($n);

    $names = array_keys((new PHPStan\Php8StubsMap($phpVersion))->functions);
    foreach (array_keys($map) as $k) {
        if (!str_contains($k, '::') && !str_contains($k, "'")) {
            $names[] = $k;
        }
    }
    $names = array_merge($names, array_keys($stubs::FUNCTIONS));
    $names = array_filter(array_unique(array_map('strtolower', $names)), fn ($n) => $provider->hasFunction($name($n), null));
    sort($names);
    $functions = [];
    foreach ($names as $n) {
        $functions[$n] = callable_entry($provider->getFunction($name($n), null));
    }

    // Constant types as a PHPDoc type names them (TypeNodeResolver -> ConstantResolver, InitializerExprTypeResolver),
    // which can be more precise than the reflection's getValueType() (typed class constants: `int` vs `16`).
    $resolver = $c->getByType(TypeStringResolver::class);
    $constants = [];
    foreach (array_keys($stubs::CONSTANTS) as $n) {
        if ($provider->hasConstant($name($n), null)) {
            $constant = $provider->getConstant($name($n), null);
            $constants[$constant->getName()] = t($resolver->resolve($constant->getName(), new NameScope(null, [])));
        }
    }
    ksort($constants);

    $classes = [];
    foreach (array_keys($stubs::CLASSES) as $n) {
        if (!$provider->hasClass($n)) {
            continue;
        }
        $class = $provider->getClass($n);
        $GLOBALS['ctx'] = $class->getName();
        $entry = [
            'name' => $class->getName(),
            // every parent class and interface (not the class itself), for isSuperTypeOf between object types
            'ancestors' => array_values(array_filter(array_map(fn ($a) => $a->getName(), $class->getAncestors()), fn ($a) => $a !== $class->getName())),
            // name => [bound, default or null]
            'templates' => array_map(fn ($t) => $t instanceof PHPStan\Type\Generic\TemplateType ? [t($t->getBound()), $t->getDefault() !== null ? t($t->getDefault()) : null] : [t($t), null], $class->getTemplateTypeMap()->getTypes()),
            'cases' => $class->isEnum() ? array_keys($class->getEnumCases()) : [],
            'constants' => [],
            'methods' => [],
        ];
        foreach ($class->getNativeReflection()->getReflectionConstants() as $const) {
            if ($const->getDeclaringClass()->getName() === $class->getName() && !($class->isEnum() && $class->hasEnumCase($const->getName()))) {
                $entry['constants'][$const->getName()] = t($resolver->resolve('\\'.$class->getName().'::'.$const->getName(), new NameScope(null, [])));
            }
        }
        foreach ($class->getNativeReflection()->getMethods() as $method) {
            if ($method->getDeclaringClass()->getName() === $class->getName()) {
                $m = $class->getNativeMethod($method->getName());
                $entry['methods'][strtolower($method->getName())] = ['static' => $m->isStatic()] + callable_entry($m);
            }
        }
        $classes[strtolower($class->getName())] = $entry;
    }
    ksort($classes);
    $GLOBALS['ctx'] = null;

    return ['functions' => $functions, 'constants' => $constants, 'classes' => $classes];
}

if (($argv[1] ?? '') === 'dump') {
    $typeStrings = [];
    $GLOBALS['reflected'] = [];
    $functions = dump((int) $argv[2], $typeStrings);
    echo serialize([$functions, $typeStrings, $GLOBALS['reflected']]);
    exit;
}

$child = fn (int $v) => unserialize(shell_exec(escapeshellarg(PHP_BINARY).' -d max_memory_limit=2G -d memory_limit=2G '.escapeshellarg(__FILE__).' dump '.$v)) ?: exit(1);
[$v84, $typeStrings, $reflected] = $child(80400);
[$v85, $typeStrings85, $reflected85] = $child(80500);
$typeStrings = array_replace_recursive($typeStrings, $typeStrings85);
$reflected = array_replace_recursive($reflected, $reflected85);
$delta = [];
foreach ($v84 as $kind => $symbols) {
    foreach ($symbols + $v85[$kind] as $name => $_) {
        if (($symbols[$name] ?? null) !== ($v85[$kind][$name] ?? null)) {
            $delta[$kind][$name] = $v85[$kind][$name] ?? null;
        }
    }
}
$symbols = $v84 + ['php85' => $delta];
$resolver = container(80400)->getByType(TypeStringResolver::class);
$described = [];
foreach ($typeStrings as $ctx => $strings) {
    foreach (array_keys($strings) as $s) {
        if ($s === '') {
            continue; // MixedType(true), not a type string
        }
        $type = $resolver->resolve((string) $s, new NameScope(null, [], $ctx === '' ? null : $ctx));
        $described[] = [$ctx === '' ? null : $ctx, (string) $s, $type->describe(VerbosityLevel::precise()), $type->describe(VerbosityLevel::value()), $type->describe(VerbosityLevel::typeOnly())];
    }
}
usort($described, fn ($a, $b) => [$a[0], $a[1]] <=> [$b[0], $b[1]]);
$reflectedList = [];
foreach ($reflected as $ctx => $types) {
    foreach ($types as $s => $descs) {
        $reflectedList[] = [$ctx === '' ? null : $ctx, (string) $s, ...array_merge(...array_map(fn ($d) => json_decode($d), array_keys($descs)))];
    }
}
usort($reflectedList, fn ($a, $b) => [$a[0], $a[1]] <=> [$b[0], $b[1]]);

$flags = JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE | JSON_THROW_ON_ERROR;
@mkdir($root.'/crates/pun-phpstan/data');
file_put_contents($root.'/crates/pun-phpstan/data/symbols.json', json_encode($symbols, $flags)."\n");
file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/type-strings.json', json_encode($described, $flags | JSON_PRETTY_PRINT)."\n");
file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/reflected-types.json', json_encode($reflectedList, $flags | JSON_PRETTY_PRINT)."\n");
echo implode(', ', array_map(fn ($k) => count($v84[$k])." $k (".count($delta[$k] ?? []).' differ on 8.5)', array_keys($v84))), ', ', count($described), ' signature-map type strings, ', count($reflectedList), " reflected types\n";
