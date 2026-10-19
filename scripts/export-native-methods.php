<?php

declare(strict_types=1);

/*
 * Built-in classes' methods as PHPStan 2.2.7's reflection gives them to OverridingMethodRule and
 * ConsistentConstructorRule (NativeMethodReflection over php-8-stubs, BetterReflection's prototypes and tentative
 * return types), from the phar's own container at phpVersion 8.5, with the 8.4 differences.
 *
 *   php scripts/export-native-methods.php   writes crates/pun-analyse/data/native-methods.json
 *
 * {"classes": {"<lowercase class>": C}, "php84": {"<lowercase class>": C|null}} where C is
 *   {"name", "parent": name|null, "interfaces": [immediate interface names], "methods": {"<lowercase name>": M}}
 * and M, for each method the class declares:
 *   {"name", "visibility": public|protected|private, "static", "abstract", "final" (isFinalByKeyword), "finalDoc"
 *    (isFinal), "builtin", "variants" (count), "params": [[name, native type, optional, variadic, byRef]],
 *    "return": native return type, "tentative"?: type, "prototype"?: class (BetterReflection's getPrototype()),
 *    "willChange"?: true (#[ReturnTypeWillChange])}, without the keys at their defaults (public, false, built-in, one
 *    variant, no parameters, implicit mixed).
 * A type is its toPhpDocNode() string; an implicit mixed (no native type) is null, `mixed` an explicit one.
 */

use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\Reflection\ExtendedFunctionVariant;
use PHPStan\Reflection\ReflectionProvider;
use PHPStan\Type\MixedType;
use PHPStan\Type\Type;
use PHPStan\Type\TypehintHelper;

$root = dirname(__DIR__);
Phar::loadPhar($root.'/.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar', 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';

function container(int $phpVersion): PHPStan\DependencyInjection\Container
{
    $tmp = sys_get_temp_dir().'/pun-native-methods-'.$phpVersion;
    @mkdir($tmp);
    $neon = $tmp.'/v.neon';
    file_put_contents($neon, "parameters:\n\tphpVersion: $phpVersion\n");

    return (new ContainerFactory($tmp))->create($tmp, [$neon], []);
}

function t(Type $type): ?string
{
    if ($type instanceof MixedType && !$type->isExplicitMixed()) {
        return null;
    }

    return (string) $type->toPhpDocNode();
}

function dump(int $phpVersion): array
{
    $provider = container($phpVersion)->getByType(ReflectionProvider::class);
    $stubs = _PHPStan_88a9c7bcc\JetBrains\PHPStormStub\PhpStormStubsMap::class;
    $classes = [];
    foreach (array_keys($stubs::CLASSES) as $n) {
        if (!$provider->hasClass($n)) {
            continue;
        }
        $class = $provider->getClass($n);
        $native = $class->getNativeReflection();
        $entry = [
            'name' => $class->getName(),
            'parent' => $class->getParentClass()?->getName(),
            'interfaces' => array_values(array_map(fn ($i) => $i->getName(), $class->getImmediateInterfaces())),
            'methods' => [],
        ];
        foreach ($native->getMethods() as $method) {
            if ($method->getDeclaringClass()->getName() !== $class->getName()) {
                continue;
            }
            $m = $class->getNativeMethod($method->getName());
            $variants = $m->getVariants();
            $variant = $variants[0];
            $abstract = $m->isAbstract();
            $e = [
                'name' => $m->getName(),
                'visibility' => $m->isPrivate() ? 'private' : ($m->isPublic() ? 'public' : 'protected'),
                'static' => $m->isStatic(),
                'abstract' => is_bool($abstract) ? $abstract : $abstract->yes(),
                'final' => $m->isFinalByKeyword()->yes(),
                'finalDoc' => $m->isFinal()->yes(),
                'builtin' => is_bool($m->isBuiltin()) ? $m->isBuiltin() : $m->isBuiltin()->yes(),
                'variants' => count($variants),
                'params' => array_map(fn ($p) => [$p->getName(), t($p->getNativeType()), $p->isOptional(), $p->isVariadic(), !$p->passedByReference()->no()], $variant->getParameters()),
                'return' => $variant instanceof ExtendedFunctionVariant ? t($variant->getNativeReturnType()) : null,
            ];
            if ($method->getTentativeReturnType() !== null) {
                $e['tentative'] = t(TypehintHelper::decideTypeFromReflection($method->getTentativeReturnType(), selfClass: $class));
            }
            try {
                $e['prototype'] = $method->getPrototype()->getDeclaringClass()->getName();
            } catch (ReflectionException) {
            }
            if (count($method->getAttributes('ReturnTypeWillChange')) > 0) {
                $e['willChange'] = true;
            }
            // defaults left out: public, not static/abstract/final, built-in, one variant, no parameters, implicit mixed
            $defaults = ['visibility' => 'public', 'static' => false, 'abstract' => false, 'final' => false, 'finalDoc' => false, 'builtin' => true, 'variants' => 1, 'params' => [], 'return' => null];
            $entry['methods'][strtolower($m->getName())] = array_filter($e, fn ($v, $k) => !array_key_exists($k, $defaults) || $defaults[$k] !== $v, ARRAY_FILTER_USE_BOTH);
        }
        $classes[strtolower($class->getName())] = $entry;
    }
    ksort($classes);

    return $classes;
}

$classes = dump(80500);
$old = dump(80400);
$php84 = [];
foreach (array_unique(array_merge(array_keys($classes), array_keys($old))) as $k) {
    if (($classes[$k] ?? null) !== ($old[$k] ?? null)) {
        $php84[$k] = $old[$k] ?? null;
    }
}
ksort($php84);
file_put_contents(
    $root.'/crates/pun-analyse/data/native-methods.json',
    json_encode(['classes' => $classes, 'php84' => (object) $php84], JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE | JSON_THROW_ON_ERROR)."\n",
);
