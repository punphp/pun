<?php

declare(strict_types=1);

/*
 * Built-in classes' methods, properties and constant native types as PHPStan 2.3.5's reflection gives them to the
 * overriding rules (NativeMethodReflection over php-8-stubs, BetterReflection's prototypes and tentative return
 * types), from the phar's own container at phpVersion 8.5, with the 8.4 differences.
 *
 *   php scripts/export-native-methods.php   writes crates/pun-analyse/data/native-methods.json
 *
 * {"classes": {"<lowercase class>": C}, "php84": {"<lowercase class>": C|null}} where C is
 *   {"name", "parent": name|null, "interfaces": [immediate interface names], "methods": {"<lowercase name>": M},
 *    "properties"?: {"<name>": P}, "constantTypes"?: {"<name>": native type}}
 * and M, for each method the class declares:
 *   {"name", "visibility": public|protected|private, "static", "abstract", "final" (isFinalByKeyword), "finalDoc"
 *    (isFinal), "builtin", "variants" (count), "params": [[name, native type, optional, variadic, byRef]],
 *    "return": native return type, "tentative"?: type, "prototype"?: class (BetterReflection's getPrototype()),
 *    "willChange"?: true (#[ReturnTypeWillChange])}, without the keys at their defaults (public, false, built-in, one
 *    variant, no parameters, implicit mixed).
 * and P, for each non-private property the class declares: {"visibility", "static", "readonly", "abstract", "final"
 *    (isFinalByKeyword), "finalDoc" (isFinal), "virtual", "hooks": [get|set], "readable", "writable", "type": native type},
 *    without the keys at their defaults (public, false, no hooks, readable and writable, no native type).
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

/** PHPStan 2.3.4+ prints class names of toPhpDocNode() as name nodes (`\\Foo`); Pun's data keeps them bare, as the
 * PHPDoc parser reads them (the same type either way). */
function phpdoc_string(PHPStan\PhpDocParser\Ast\Type\TypeNode $node): string
{
    [$node] = (new PHPStan\PhpDocParser\Ast\NodeTraverser([new class extends PHPStan\PhpDocParser\Ast\AbstractNodeVisitor {
        public function leaveNode(PHPStan\PhpDocParser\Ast\Node $node)
        {
            return $node instanceof PHPStan\PhpDocParser\Ast\Type\NameNode ? new PHPStan\PhpDocParser\Ast\Type\IdentifierTypeNode($node->toString()) : null;
        }
    }]))->traverse([$node]);

    return (string) $node;
}
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

    return phpdoc_string($type->toPhpDocNode());
}

function dump(int $phpVersion): array
{
    $provider = container($phpVersion)->getByType(ReflectionProvider::class);
    $stubs = _PHPStan_95d9e8b05\JetBrains\PHPStormStub\PhpStormStubsMap::class;
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
        foreach ($native->getProperties() as $property) {
            if ($property->getDeclaringClass()->getName() !== $class->getName() || $property->isPrivate()) {
                continue;
            }
            $p = $class->getNativeProperty($property->getName());
            $e = [
                'visibility' => $p->isPublic() ? 'public' : 'protected',
                'static' => $p->isStatic(),
                'readonly' => $p->isReadOnly(),
                'abstract' => $p->isAbstract()->yes(),
                'final' => $p->isFinalByKeyword()->yes(),
                'finalDoc' => $p->isFinal()->yes(),
                'virtual' => $p->isVirtual()->yes(),
                'hooks' => array_values(array_filter(['get', 'set'], fn ($h) => $p->hasHook($h))),
                'readable' => $p->isReadable(),
                'writable' => $p->isWritable(),
                'type' => $p->hasNativeType() ? t($p->getNativeType()) : null,
            ];
            $defaults = ['visibility' => 'public', 'static' => false, 'readonly' => false, 'abstract' => false, 'final' => false, 'finalDoc' => false, 'virtual' => false, 'hooks' => [], 'readable' => true, 'writable' => true, 'type' => null];
            $entry['properties'][$property->getName()] = (object) array_filter($e, fn ($v, $k) => $defaults[$k] !== $v, ARRAY_FILTER_USE_BOTH);
        }
        foreach ($native->getReflectionConstants() as $constant) {
            if ($constant->getDeclaringClass()->getName() !== $class->getName()) {
                continue;
            }
            $type = $class->getConstant($constant->getName())->getNativeType();
            if ($type !== null) {
                $entry['constantTypes'][$constant->getName()] = t($type);
            }
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
    if (json_encode($classes[$k] ?? null) !== json_encode($old[$k] ?? null)) {
        $php84[$k] = $old[$k] ?? null;
    }
}
ksort($php84);
file_put_contents(
    $root.'/crates/pun-analyse/data/native-methods.json',
    json_encode(['classes' => $classes, 'php84' => (object) $php84], JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE | JSON_THROW_ON_ERROR)."\n",
);
