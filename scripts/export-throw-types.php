<?php

declare(strict_types=1);

/*
 * Built-in functions' and methods' throw types as PHPStan 2.3.1's reflection gives them to the throw points
 * (FunctionReflection::getThrowType() and MethodReflection::getThrowType(): functionMap, php-8-stubs and
 * stubs/*.stub PHPDocs), from the phar's own container at phpVersion 8.5.
 *
 *   php scripts/export-throw-types.php   writes crates/pun-analyse/data/throw-types.json
 *
 * {"functions": {"<lowercase name>": T}, "methods": {"<lowercase class>": {"<lowercase method>": T}},
 *  "dynamic": ["<lowercase function>" | "<lowercase class>::<lowercase method>"],
 *  "later": {"<lowercase function>": [parameter names]}, "immediate": {"<lowercase class>::<lowercase method>": [...]}}
 * where T is the throw type's toPhpDocNode() string (`void` for `@throws void`); members without one are left out,
 * as are methods only inherited from a parent. `dynamic` lists the members a dynamic throw type extension decides
 * the throw point of (Pun reads those as unknown). `later` are the functions' parameters whose
 * isImmediatelyInvokedCallable() is no, `immediate` the methods' parameters where it is yes (ArgumentsHandler's
 * callCallbackImmediately()).
 */

use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\Reflection\ReflectionProvider;
use PHPStan\Type\DynamicFunctionThrowTypeExtension;
use PHPStan\Type\DynamicMethodThrowTypeExtension;
use PHPStan\Type\DynamicStaticMethodThrowTypeExtension;

$root = dirname(__DIR__);
Phar::loadPhar($root.'/.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar', 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';

$tmp = sys_get_temp_dir().'/pun-throw-types';
@mkdir($tmp);
file_put_contents($tmp.'/v.neon', "parameters:\n\tphpVersion: 80500\n");
$container = (new ContainerFactory($tmp))->create($tmp, [$tmp.'/v.neon'], []);
$provider = $container->getByType(ReflectionProvider::class);
$functionExtensions = $container->getServicesByTag('phpstan.dynamicFunctionThrowTypeExtension');
$methodExtensions = $container->getServicesByTag('phpstan.dynamicMethodThrowTypeExtension');
$staticExtensions = $container->getServicesByTag('phpstan.dynamicStaticMethodThrowTypeExtension');

$stubs = _PHPStan_cb3b7e994\JetBrains\PHPStormStub\PhpStormStubsMap::class;
$symbols = json_decode(file_get_contents($root.'/crates/pun-phpstan/data/symbols.json'), true);

$functions = [];
$dynamicMembers = [];
$later = [];
$immediate = [];
foreach (array_keys($symbols['functions']) as $name) {
    $n = new PhpParser\Node\Name($name);
    if (!$provider->hasFunction($n, null)) {
        continue;
    }
    $f = $provider->getFunction($n, null);
    $dynamic = array_filter($functionExtensions, fn (DynamicFunctionThrowTypeExtension $e) => $e->isFunctionSupported($f));
    if ($dynamic !== []) {
        $dynamicMembers[] = strtolower($f->getName());
    }
    foreach ($f->getVariants() as $variant) {
        foreach ($variant->getParameters() as $p) {
            if ($p->isImmediatelyInvokedCallable()->no()) {
                $later[strtolower($f->getName())][] = $p->getName();
            }
        }
    }
    $type = $f->getThrowType();
    if ($type !== null) {
        $functions[strtolower($f->getName())] = (string) $type->toPhpDocNode();
    }
}
ksort($functions);

$methods = [];
foreach (array_keys($stubs::CLASSES) as $n) {
    if (!$provider->hasClass($n)) {
        continue;
    }
    $class = $provider->getClass($n);
    foreach ($class->getNativeReflection()->getMethods() as $method) {
        if ($method->getDeclaringClass()->getName() !== $class->getName()) {
            continue;
        }
        $m = $class->getNativeMethod($method->getName());
        $dynamic = array_filter($methodExtensions, fn (DynamicMethodThrowTypeExtension $e) => $e->isMethodSupported($m))
            + array_filter($staticExtensions, fn (DynamicStaticMethodThrowTypeExtension $e) => $e->isStaticMethodSupported($m));
        $key = strtolower($class->getName()).'::'.strtolower($m->getName());
        if ($dynamic !== []) {
            $dynamicMembers[] = $key;
        }
        foreach ($m->getVariants() as $variant) {
            foreach ($variant->getParameters() as $p) {
                if ($p->isImmediatelyInvokedCallable()->yes()) {
                    $immediate[$key][] = $p->getName();
                }
            }
        }
        $type = $m->getThrowType();
        if ($type !== null) {
            $methods[strtolower($class->getName())][strtolower($m->getName())] = (string) $type->toPhpDocNode();
        }
    }
}
ksort($methods);
sort($dynamicMembers);
ksort($later);
ksort($immediate);
foreach ($methods as &$m) {
    ksort($m);
}
unset($m);

file_put_contents(
    $root.'/crates/pun-analyse/data/throw-types.json',
    json_encode(['functions' => $functions, 'methods' => $methods, 'dynamic' => array_values(array_unique($dynamicMembers)), 'later' => array_map(fn ($p) => array_values(array_unique($p)), $later), 'immediate' => array_map(fn ($p) => array_values(array_unique($p)), $immediate)], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR)."\n",
);
