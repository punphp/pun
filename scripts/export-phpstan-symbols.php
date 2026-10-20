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
 *   __stringAndStringable, __stringNotStringable         the string types around objects with __toString()
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
        if ($t instanceof PHPStan\Type\StringAlwaysAcceptingObjectWithToStringType) {
            return $hole('__stringAndStringable');
        }
        if ($t instanceof PHPStan\Type\StringNeverAcceptingObjectWithToStringType) {
            return $hole('__stringNotStringable');
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
            'interface' => $class->isInterface(),
            'final' => $class->isFinalByKeyword(),
            // public instance properties, declared or inherited
            'properties' => array_values(array_map(fn ($p) => $p->getName(), array_filter($class->getNativeReflection()->getProperties(ReflectionProperty::IS_PUBLIC), fn ($p) => !$p->isStatic()))),
            'constants' => [],
            'methods' => [],
        ];
        // the templates' variance, when not invariant (TemplateTypeVariance::describe())
        foreach ($class->getTemplateTypeMap()->getTypes() as $name => $t) {
            if ($t instanceof PHPStan\Type\Generic\TemplateType && !$t->getVariance()->invariant()) {
                $entry['variances'][$name] = $t->getVariance()->describe();
            }
        }
        // getIterableKeyType() / getIterableValueType() over the class's own templates (for foreach and iterable<>)
        if ($class->implementsInterface(Traversable::class) || $class->getName() === Traversable::class) {
            $own = array_values($class->getTemplateTypeMap()->getTypes());
            $object = $own === [] ? new PHPStan\Type\ObjectType($class->getName()) : new PHPStan\Type\Generic\GenericObjectType($class->getName(), $own);
            $entry['iterable'] = [t($object->getIterableKeyType()), t($object->getIterableValueType())];
        }
        // generic ancestors' type arguments over the class's own templates (GenericObjectType::isSuperTypeOf). Not
        // reflected types: an inherited `static` stays the declaring class's (SimpleXMLIterator's
        // `static(SimpleXMLElement)`), which Pun reads as this class's.
        $reflected = $GLOBALS['reflected'];
        $own = array_values($class->getTemplateTypeMap()->getTypes());
        $object = $own === [] ? new PHPStan\Type\ObjectType($class->getName()) : new PHPStan\Type\Generic\GenericObjectType($class->getName(), $own);
        foreach ($entry['ancestors'] as $ancestor) {
            $generic = $object->getAncestorWithClassName($ancestor);
            if ($generic instanceof PHPStan\Type\Generic\GenericObjectType) {
                $entry['generics'][$ancestor] = array_map('t', $generic->getTypes());
            }
        }
        $GLOBALS['reflected'] = $reflected;
        foreach ($class->getNativeReflection()->getReflectionConstants() as $const) {
            if ($const->getDeclaringClass()->getName() === $class->getName() && !($class->isEnum() && $class->hasEnumCase($const->getName()))) {
                $entry['constants'][$const->getName()] = t($resolver->resolve('\\'.$class->getName().'::'.$const->getName(), new NameScope(null, [])));
            }
        }
        foreach ($class->getNativeReflection()->getMethods() as $method) {
            if ($method->getDeclaringClass()->getName() === $class->getName()) {
                $m = $class->getNativeMethod($method->getName());
                $entry['methods'][strtolower($method->getName())] = ['static' => $m->isStatic()] + callable_entry($m);
                // the PHPDoc a user method overriding this one inherits (PhpDocInheritanceResolver): its
                // `@param` types by parameter position and its `@return` type
                $doc = $m->getResolvedPhpDoc();
                if ($doc !== null) {
                    $positions = array_map(fn ($p) => $p->getName(), $method->getParameters());
                    $params = [];
                    foreach ($doc->getParamTags() as $paramName => $tag) {
                        $i = array_search($paramName, $positions, true);
                        if ($i !== false) {
                            $params[(string) $i] = t($tag->getType());
                        }
                    }
                    ksort($params);
                    $return = $doc->getReturnTag();
                    $entry['methods'][strtolower($method->getName())]['doc'] = [
                        'params' => (object) $params,
                        'return' => $return !== null ? t($return->getType()) : null,
                    ];
                }
            }
        }
        // the `@var` type of each declared, non-private property with one (inherited by a user property)
        foreach ($class->getNativeReflection()->getProperties() as $property) {
            if ($property->getDeclaringClass()->getName() !== $class->getName() || $property->isPrivate()) {
                continue;
            }
            $p = $class->getNativeProperty($property->getName());
            if ($p->hasPhpDocType()) {
                $entry['propertyDocs'][$property->getName()] = t($p->getPhpDocType());
            }
        }
        $classes[strtolower($class->getName())] = $entry;
    }
    ksort($classes);
    $GLOBALS['ctx'] = null;

    return ['functions' => $functions, 'constants' => $constants, 'classes' => $classes];
}


/*
 * type-probes.json: hand-written type strings (not Pun's own extensions, which PHPStan doesn't read) for the resolver and type-system paths built-in signatures don't
 * reach (written as PHPDoc in user code), resolved the same way. `php scripts/export-phpstan-symbols.php probes`
 * rewrites only this file.
 */
const PROBES = [
    'non-zero-int', 'enum-string', 'numeric', 'number', 'scalar', 'non-empty-lowercase-string', 'non-empty-uppercase-string',
    'non-empty-literal-string', 'callable-object', 'callable-array', 'non-empty-list', 'truthy-string', 'double', 'boolean',
    'integer', 'associative-array', 'never-return', 'noreturn', 'open-resource', 'interface-string', 'trait-string',
    'callable-string', 'negative-int', 'non-positive-int', 'non-negative-int', 'literal-string', 'uppercase-string',
    'iterable', 'object', 'void', 'never', 'self', 'static', '$this', 'Foo-bar', 'OCI-Lob', 'NOT_A_CONSTANT', 'Foo', '\\Countable',
    'TRUE', 'PHP_SAPI', 'PHP_INT_MIN', 'INF', 'NAN', 'M_PI', 'PHP_FLOAT_EPSILON', 'PHP_EOL', 'E_ALL',
    "array<'a', int>", 'array<1, int>', 'array<int, int, int>', 'array<float, int>', 'array<bool, int>', 'array<null, int>',
    "array<'1', int>", 'array<array-key, int>', 'array<mixed, int>', 'array<int|string, int>', 'non-empty-array<int>',
    'non-empty-array<string, int>', 'list<int, int>', 'non-empty-list<string>', 'iterable<int>', 'iterable<int, string>',
    'iterable<int, string, bool>', 'class-string<Countable>', 'class-string<int>', 'class-string<mixed>',
    'interface-string<Countable>', 'class-string<Countable|Traversable>', 'int<1, 3>', 'int<min, 5>', 'int<5, max>',
    'int<foo, 5>', 'int<1, bar>', 'int<1>', 'int<3, 1>', 'int<2, 2>', 'int-mask<1, 2, 4>', 'int-mask<1, 4>', 'int-mask<0, 8>',
    "int-mask<'a'>", 'int-mask-of<1|2>', 'int-mask-of<string>', 'int-mask-of<1, 2>', '__benevolent<int|string>', '__benevolent<int>',
    '__benevolent<int, string>', 'Traversable<int>', 'Iterator<int, string>', 'IteratorAggregate<string>', 'Generator<int>',
    'Generator<int, string>', 'Generator<int, string, bool, void>', 'ArrayIterator<int>', 'ArrayIterator<int, string>',
    'ArrayObject<string>', 'SplObjectStorage<Countable, int>', 'Countable<int>', 'Foo<int>', 'WeakReference<stdClass>',
    'int<1, 3>[]', 'ArrayAccess<int, string>', 'SplFixedArray<int>', 'DatePeriod', 'Closure<int>', 'int<1, 2, 3>',
    'callable(int): void', 'callable(int=, string...): mixed', 'callable(int &$x): bool', 'Closure(int): string',
    'Closure(int $a, string ...$b): (int|string)', 'Foo(int): void',
    'callable(): callable(): int', 'callable(int|string): (int|false)', 'Closure', 'callable(): void|int', 'callable(Countable): int',
    "array{DateTimeInterface::ATOM: int}", "int['a']", 'null::FOO', 'callable(int): void|callable(string): void', 'callable(): int|callable(): string', 'callable(): mixed|Closure(): int',
    'callable(int, int): int|callable(int): int', 'callable(int...): int|callable(int): int', 'callable&string', 'callable&object',
    'callable&Countable', 'callable-string|Closure', 'callable|string', 'callable(): int|string',
    'array{a: int, b?: string}', 'array{int, string}', 'list{int, string}', 'non-empty-array{a?: int}', 'non-empty-list{0?: int}',
    'non-empty-list{int}', 'array{a: int, ...}', 'array{a: int, ...<string, int>}', 'array{a: int, ...<int>}', 'list{int, ...<string>}',
    'list{int, ...}', 'array{...<int>}', 'array{...<string, int>}', 'array{...}', 'list{...}', "array{'a b': int, \"c\": int, 'd\\'e': int, 'f\"g': int}",
    'array{1: int, 3: int}', 'array{-1: int}', 'array{0: int, 2: int}', 'array{0?: int, 1?: int}', 'array{0: int, 1?: int, 2?: int}',
    'array{a: int, a: string}', 'array{a: int, a?: string}', 'array{a?: int, a?: string}', 'array{int, a: string, int}',
    'object{a: int, b?: string}', "object{'a': int}", 'array{}', 'list{}', 'array{0?: int}', 'array{a?: int}',
    'array{a: 1, b: 2, c: 3, d: 4, e: 5, f: 6, g: 7, h: 8, i: 9}', 'array{1, 2, 3, 4, 5, 6, 7, 8, 9}',
    'array{a: int, ...<1|2, string>}', 'array{a: int, ...<\'a\'|\'b\', string>}', 'array{"x": int}', 'array{1.5: int}', 'array{_a1: int, a-b: int, \\a: int, 1a: int}',
    'int|string[]', 'Traversable|string[]', 'array|int[]', 'iterable|int[]', 'list<int>|string[]', 'array{a: int}|int[]',
    'ArrayIterator<int>|int[]', 'Countable|int[]', '(int|string)[]', 'int[]|string[]', '?int', 'int|null|false', '1|2|3', '1.5|2.0|1',
    "'a'|'b'|string", 'true|false', 'true|bool', 'int<1, 3>|int<2, 5>', 'int<1, 3>|int<5, 7>', 'int<1, 3>|int<4, 7>', 'int<1, 3>|4',
    'int<1, 3>|0', 'int<1, 3>|9', 'int|int<1, 3>', 'int<1, 3>|int', '0|1|positive-int', "non-empty-string|''", "non-falsy-string|''",
    "''|non-empty-lowercase-string", "''|non-falsy-string", 'iterable<int>|iterable<string>', 'array{a: int}|array{a: string}',
    'array{a: int}|array{b: int}', 'array{}|array{a: int}', 'array{}|array{a?: int}', 'list<int>|array{a: int}', 'callable|Closure',
    'Countable|ArrayAccess', 'Exception|Throwable', 'Throwable|Exception', 'mixed|int', 'never|int', 'int|float|string|bool|null|array|object|resource', 'non-empty-array<int>|array{a: int}', 'non-empty-list<int>|array{a: int}',
    'array{a: int, b: int}|array{a: string}', 'array{a: int}|array{a: string, b?: int}', 'array{a: int, b: int}|array{a: string, c: int}',
    'array{a: int, b: string}|array{a: int, b: int}', 'list{int}|list{int, int}', 'array{0: int}|list{string, string}',
    'list<int>|list<string>', 'non-empty-list<int>|non-empty-list<string>', 'non-empty-array<int>|array<string>',
    'array<int>|array{}', 'array{}|list<int>', 'array{}|non-empty-list<int>', 'array{a: 1}|array{a: 1}', 'array{1, 2}|array{3}|array{a: 4}',
    'array<string, int>|array<int, string>', 'string|class-string', 'class-string|literal-string',
    "class-string|'Countable'", "'Countable'|class-string<Countable>", "class-string<Countable>|'Countable'", "'Countable'|literal-string",
    'numeric-string|int', "numeric-string|'1'", "'abc'|numeric-string", 'lowercase-string|uppercase-string', 'Countable|object',
    'object|Countable', 'Countable|static', 'stdClass|object{a: int}', 'object{a: int}|object{b: int}', 'object{a: int}|object',
    'Iterator|IteratorAggregate|Traversable', 'Traversable|Iterator', 'Iterator<int>|Traversable<string>', '1|1.0', "'1'|1",
    '-1|-2|0|-1.5|1.5', '1.0E+25|0.1|-0.0', 'Countable::class|Traversable::class', "'aaaaaaaaaaaaaaaaaaaaaaaaaaaa'|'b c d e f g h i j k l m n o p'",
    "'a\\tb'", '"a\nb\\\\c\"d\x07"', "'it\\'s'", "'\\\\'", '"\\101\\x41"', '"\\u{41}"', "'abcdefghijklmnopqrstuvwxyz'", "'abc defgh ijklmnopqrs tuvwxyz'",
    "'Countable'", "'countable'", "'0'", "''", '0.5', '1e3', '-1.5e-7', '0x1A', '0b101', '0o17', '017', '1_000', '-0', '9223372036854775807',
    'float|1.5', 'int|1', 'bool|true', 'string|\'a\'', 'array-key|int', 'int|array-key', '__benevolent<int|string>|int', '__benevolent<int|string>|float',
    'resource|null', 'void|null', 'Closure(): int|Closure(): string', 'Closure(int): void|callable(int): void',
    'Countable&Traversable', 'non-empty-string&numeric-string', 'string&non-empty-string', 'array<int>&non-empty-array',
    'list<int>&non-empty-array', 'array{a?: int}&non-empty-array', 'array{0?: int}&list', 'array{a: int}&list', 'int&string',
    'int<1, 5>&int<3, 9>', 'int&int<1, 3>', 'int<1, 3>&1', 'int<1, 3>&int<5, 7>', 'iterable<int>&array', 'array&iterable<int>',
    'Foo&Bar', 'callable&array', 'Traversable&iterable<int>', 'non-empty-array&list', 'non-empty-array&array{0?: int, 1?: int}',
    'array<int>&array<string>', 'array<int, int>&list', 'array<string, int>&list', 'array<int>&list<int>', 'iterable<int>&iterable<string>',
    'class-string<Countable>&class-string<Traversable>', 'non-empty-string&non-falsy-string', 'non-falsy-string&non-empty-string',
    'lowercase-string&uppercase-string&non-empty-string', 'literal-string&numeric-string', 'non-empty-string&string', 'string&int', 'Countable&object',
    'object&Countable', 'array{a: int}&array<string, int>', 'array{a: int}&array{a: int}', 'array{a: int}&array{b: int}', 'array{}&non-empty-array',
    'non-empty-array&array{}', 'mixed&int', 'int&mixed', '(int|string)&int', 'int&(1|2|"a")', '(1|2)&(2|3)', '__benevolent<int|string>&int',
    'Countable&(Traversable|ArrayAccess)', 'stdClass&object{a: int}', 'object{a: int}&stdClass', 'DateInterval&object{days: int}',
    'callable-string&non-empty-string', 'non-empty-array&callable', 'list<int>&array<int>', 'non-empty-list&array{a?: int}',
    'array<string>&non-empty-array<string>', 'array<int, string>&non-empty-array<int, string>&list',
    '(T is int ? string : bool)', '($x is int ? 1 : 2)', '($x is not int ? 1 : 2)', '(int is int ? true : false)',
    'int<min, max>', 'int<min, -9223372036854775808>', 'int<9223372036854775807, max>', 'never|1', 'never|int', 'never|Countable',
    'non-empty-string&class-string', 'class-string&non-falsy-string', 'class-string&numeric-string', 'non-falsy-string&numeric-string',
    "uppercase-string&'ABC'", "'ABC'&uppercase-string", "'abc'&uppercase-string", "lowercase-string&'abc'", "'strlen'&callable",
    "'nope'&callable", 'callable&Closure', 'callable&stdClass', 'callable&Generator', 'callable&ArrayIterator', 'Closure&callable(int): void',
    '(Countable&Traversable)|(Countable&ArrayAccess)', 'Generator&Countable', 'Countable&Generator', 'Exception&Countable', 'Exception&Error',
    'callable(int<1, 3>, \'a\', 1.5, class-string<Countable>, ArrayIterator<int, string>, iterable<int>, iterable<int, string>, iterable, list<int>, array<string, int>, array, never): void',
    'callable(callable(int): void, Closure(int $x): void, callable): void', 'callable(Countable&Traversable): void', 'callable(array<int>, array{a: int}): void',
    'callable(int...): void', 'callable(int=): void', 'callable(): (callable(): int)', 'callable(): (Countable&Traversable)', 'callable(): Closure(): int',
    'callable(int|string, int|string[]): void', 'callable(class-string, -5, 1.0E+25, \'it\\\'s\'): void', 'callable(callable(int &$x, int ...$y): void, callable(int=): void): void',
    'callable(callable(): (int|string)): void', 'callable(non-empty-string, list<int>|false, (int|string)[]): void', 'callable(object, resource, mixed, void, null): void',
    'callable(static, $this): void', 'callable(Traversable<int>, Generator<int, string>): void', 'callable(non-empty-array<int>, int<min, 5>): void',
    'callable(): iterable<int, string>', 'callable(): iterable<string>', 'callable(): iterable', '"\x08\x01"', "'a\\\\b'",
    '1.0|1', '2.0|1|1.5', '1|2.0|1.0', '1.5|1', 'int<1, 3>|int<min, 0>', 'int<5, max>|int<1, 3>', 'int<1, 3>|int<min, max>', 'int<1, 3>|1|2|3',
    '1|2|3|int<1, 3>', 'int<1, 2>|int<2, 3>|int<9, 10>', 'array{a: int}|array{a: int, b?: int}', 'array{a?: int}|array{}', 'array{a: int, b?: int}|array{a: int}',
    'array{a: int, b: int}|array{a: int}', 'array{a: int}|array{a: int, b: int}', 'array{a: int, b: int, c: int}|array{a: int}',
    'array{a: int, b: int}|array{b: string, a: string}', 'array{0: int}|array{0: string}', 'list{int}|list{string}', 'array{a: 1, b: 2}|array{a: 1, b: 3}|array{a: 2, b: 2}',
    'array<int, int>|array{a: int}', 'array{a: int}|array<string, int>|false', 'non-empty-array{a: int}|non-empty-array{b: int}', 'list{0?: int}|list{int, int}',
    'array{0?: int, 1?: int}&non-empty-array', 'list{0?: int, 1?: int}&non-empty-array', 'array{a?: int, b?: int}&non-empty-array', 'array{a?: int, ...}&non-empty-array',
    'array{0: int}&list', 'array{1: int}&list', 'array{a: int, ...}&list', 'array<int>&list', 'array<int<0, max>, int>&list', 'array<1|2, int>&list', "array<'a', int>&list",
    'array{a: int}&array<int, int>', 'array{a: int, b?: string}&array<string, int>', 'array{a: int, ...}&array<string, int>',
    'iterable<int>&iterable<int>', 'iterable<int>&Traversable', 'iterable<int>&Countable', 'iterable&array<int>', 'iterable<int, string>&array', 'list<int>&iterable<string>',
    'mixed|int', 'mixed', 'non-empty-mixed', 'array<int, mixed>', 'array<mixed>', 'iterable<mixed, mixed>', 'iterable<mixed>', 'list<mixed>',
    'class-string<Countable>|class-string<Traversable>', "class-string<Countable>&'Countable'", "'Countable'&class-string<Countable>", "'ArrayIterator'&class-string<Countable>",
    "'Nope'&class-string", "'Countable'&class-string", 'class-string&string', 'class-string<mixed>&class-string', "'Countable'|class-string<mixed>", "class-string<mixed>|'Countable'",
    "'Countable'&class-string<object>", 'class-string<object>|class-string', "class-string|class-string<Countable>",
    'array{}|array{}', 'array{}&array{}', 'array{}&list', 'array{}&array<int>', 'array<int>&array{}', 'array{a: int}&array{}', 'list{int, string}&list', 'list{int, string}&array{int, string}',
    'array{a: int}&array{a: 1}', 'array{a: int, b: int}&array{a: int}', 'array{a: int}&array{a?: int}', 'array{a: int}&array{a: string}',
    'Closure(int): void|Closure(string): void', 'Closure|Closure(): void', 'Closure(): void|Closure', 'Closure(): void&Closure', 'callable&Closure(): void',
    'Iterator&Traversable<int>', 'Traversable<int>&Iterator', 'ArrayIterator<int, string>&Countable', 'ArrayIterator<int, string>|ArrayIterator<int, string>',
    'ArrayIterator<int, string>|ArrayIterator<int, int>', 'ArrayIterator<int, string>|ArrayIterator', 'ArrayIterator|ArrayIterator<int, string>',
    'ArrayIterator<int, string>|SeekableIterator<int, string>', 'SeekableIterator<int, string>|ArrayIterator<int, string>', 'Iterator<int, string>|ArrayIterator<int, string>',
    'ArrayIterator<int, string>|Iterator<int, string>', 'ArrayIterator<int, string>|Countable', 'Countable|ArrayIterator<int, string>', 'ArrayIterator<Foo, string>|ArrayIterator<Bar, string>',
    'Countable|Traversable|Countable', 'stdClass|object{a: int}|object', 'object{a: int}|stdClass', 'object{a: int}&object{b: int}', 'object{a: int}&object{a: int}', 'object{a: int}&object',
    'object&object{a: int}', 'object{a: int}|object{a: int}', 'Countable&object{a: int}', 'object{a: int}&Countable', 'DateInterval|object{days: int}',
    'iterable<int>|array<int>', 'array<int>|iterable<int>', 'iterable<int>|Traversable<int>', 'Traversable<int>|iterable<int>', 'iterable<int>|list<int>', 'iterable<int>|int',
    'Countable|iterable<int>', 'iterable<int>|Countable', 'iterable<int>|ArrayIterator<int, int>', 'iterable<int, string>|iterable<string, int>', 'iterable|iterable<int>',
    '(T is int ? string : bool)|int', 'int|(T is int ? string : bool)', 'mixed|(T is int ? string : bool)', '(T is int ? string : bool)&int',
    "''|non-empty-string", "''|non-falsy-string|'a'", "''|lowercase-string&non-empty-string", "''|uppercase-string&non-falsy-string", "''|numeric-string",
    "non-empty-string|'0'", "'0'|non-falsy-string", "non-falsy-string|'0'|''", 'positive-int|0', 'negative-int|0|positive-int',
    'callable-string|non-empty-string', 'non-empty-string|callable-string', 'callable|int', 'callable|callable', 'callable(): int|callable(): int',
    'array-key|float', '__benevolent<int|string>|null', '__benevolent<int|string|null>', '__benevolent<int|string>|bool', 'scalar|null', 'null|scalar',
    'Countable|null|Traversable', 'null|Countable', 'Countable|null', 'null', '?null', '?mixed', '?Countable', '?callable', '?(int|string)', '?int[]', '?array{a: int}',
    'resource|resource', 'void|void', 'object|object', 'object|stdClass', 'stdClass|object', 'Countable|object|Traversable', 'stdClass&object', 'object&stdClass',
    '\\stdClass', 'STDCLASS', 'stdclass|stdClass', 'stdClass&stdclass', 'Exception|RuntimeException', 'RuntimeException|Exception', 'RuntimeException&Exception',
    'Exception&RuntimeException', 'RuntimeException&LogicException', 'Throwable&RuntimeException', 'Countable&RuntimeException', 'RuntimeException&Countable',
    'Closure&Countable', 'Generator&Iterator', 'WeakMap&Countable', 'Countable&WeakMap', 'Closure&Exception',
    'array{0: Countable, ...<int, non-falsy-string>}&callable', 'array{...<string, string>}&callable', 'array{0: Countable, ...<int, int>}&callable',
    'array{1: string, ...<int, Countable>}&callable', "array{'Countable', 'count'}&callable", 'array{1, 2}&callable', 'array{1, 2, 3}&callable',
    'array{a: 1, b: 2}&callable', 'array{0: Countable, ...<int, string>}&callable', 'array{Countable}&callable', 'array<string>&callable',
    'iterable<int>&callable', 'iterable<int>&never', 'array<int>&never', 'never&Countable',
    // generalize() through constant-array values and intersection members
    "array{a: Closure(1): 2, b: Generator<1, 'a'>, c: class-string<stdClass>, d: (T is 1 ? 2 : 3), e: (\$x is 1 ? 1.5 : 2.5), f: array<int, true>, g: iterable<1>}|null",
    "array{a: Closure(1): 2}|1", "array{a: true}|array{b: 'x'}|1", "Countable&Generator<1, 'a'>", 'Countable&Iterator<1, true>',
    "array{a: int, ...<'b', int>}", 'array{a: int, ...<5, int>}', "array<__benevolent<'1'|1>, int>", 'array<true, int>', 'class-string<Countable&callable>',
    'int-mask<1>', 'int-mask-of<1|string>', 'string<int>', 'SimpleXMLElement<int, string>', 'SimpleXMLElement<string>', 'SimpleXMLElement<int, string, bool>',
    '__template<int>', 'non-empty-list<int>&list<int>',
    "array{a?: int}&array{}", "array{}&array{a?: int}", 'callable-array|null', 'callable-array|array{a: 1}', 'iterable<int>|callable-array',
    'callable((Countable&Traversable)|int): void', 'callable(): ((Countable&Traversable)|int)', "array{'\\\\Foo\\\\Bar': int, '\\\\': int, '\\\\1': int}",
    'array{a: int, ...<__benevolent<int|string>, int>}', 'never&never', 'never|never', 'never-return|never', 'never&never-return',
    'SplObjectStorage<*, *>', 'int|', '<', '(int', 'array{', 'callable(',
];

/** Function templates the `templates` probes resolve with (Pun reads them as `__template<...>`). */
const PROBE_TEMPLATES = ['T' => null, 'U' => 'int|string', 'V' => 'Countable'];

function probe_templates(TypeStringResolver $resolver): PHPStan\Type\Generic\TemplateTypeMap
{
    $scope = PHPStan\Type\Generic\TemplateTypeScope::createWithFunction('probe');
    $types = [];
    foreach (PROBE_TEMPLATES as $name => $bound) {
        $types[$name] = PHPStan\Type\Generic\TemplateTypeFactory::create($scope, $name, $bound === null ? null : $resolver->resolve($bound, new NameScope(null, [])), PHPStan\Type\Generic\TemplateTypeVariance::createInvariant());
    }

    return new PHPStan\Type\Generic\TemplateTypeMap($types);
}

/** @return list<array{?string, string, string, string, string, ?string, ?array{string, string}}> */
function describe_all(TypeStringResolver $resolver, array $strings): array
{
    $described = [];
    $templates = probe_templates($resolver);
    foreach ($strings as [$ctx, $s]) {
        $withTemplates = $ctx === 'templates';
        if ($withTemplates) {
            $ctx = null;
        }
        try {
            $type = $resolver->resolve($s, new NameScope(null, [], $ctx, null, $withTemplates ? $templates : null));
        } catch (Throwable $e) {
            $type = new PHPStan\Type\ErrorType(); // phpdoc-parser rejects it; PHPDoc tags with a parse error resolve to *ERROR*
        }
        $row = [$ctx, $s, $type->describe(VerbosityLevel::precise()), $type->describe(VerbosityLevel::value()), $type->describe(VerbosityLevel::typeOnly())];
        // the same type in Pun's syntax
        $row[] = $withTemplates ? preg_replace_callback('~\\b[TUV]\\b~', fn ($m) => sprintf("__template<%s, 'function probe()', %s>", $m[0], PROBE_TEMPLATES[$m[0]] ?? 'mixed'), $s) : null;
        // what foreach gives
        $row[] = $type->isIterable()->yes() ? [$type->getIterableKeyType()->describe(VerbosityLevel::precise()), $type->getIterableValueType()->describe(VerbosityLevel::precise())] : null;
        $described[] = $row;
    }

    return $described;
}

$flags = JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE | JSON_THROW_ON_ERROR;
// php scripts/export-phpstan-symbols.php accepts: tests/fixtures/type-accepts.json, every ordered pair of the atoms as
// [context, a, b, accepts(b, strict) result, accepts(b, non-strict) result, getRecommendedLevelByType(a, b)] (b null:
// getRecommendedLevelByType(a) alone; b `strict-mixed`: RuleLevelHelper's StrictMixedType), and type-casts.json,
// [context, a, toString, toNumber, toInteger, toFloat, toBitwiseNotType] each true when an ErrorType, then isCloneable()
// (null for the StrictMixedType, which the analyser tells apart itself)
if (($argv[1] ?? '') === 'accepts') {
    $resolver = container(80400)->getByType(TypeStringResolver::class);
    $templates = probe_templates($resolver);
    $pun = fn (string $s) => preg_replace_callback('~\\b[TUV]\\b~', fn ($m) => sprintf("__template<%s, 'function probe()', %s>", $m[0], PROBE_TEMPLATES[$m[0]] ?? 'mixed'), $s);
    $level = fn (VerbosityLevel $v) => $v->isTypeOnly() ? 'TypeOnly' : ($v->isValue() ? 'Value' : 'Precise');
    $atoms = ['int', '1', 'int<1, 5>', 'float', '1.5', 'bool', 'true', 'string', "'a'", "'12'", 'non-empty-string', 'numeric-string',
        'lowercase-string', 'uppercase-string', 'non-falsy-string', 'literal-string', 'decimal-int-string', 'class-string', 'null', 'resource', 'object', 'stdClass',
        'Countable', 'Exception', 'Stringable', 'GMP', 'SimpleXMLElement', 'BcMath\\Number', 'CurlHandle', 'Traversable<int>', 'ArrayIterator<int, string>',
        'Closure', 'Closure(int): void', 'callable', 'callable(int): void', 'array', 'array{}', 'array{a: int}', 'list<int>', 'non-empty-array<int>',
        'iterable<int>', 'static', '$this', 'Random\\IntervalBoundary::ClosedOpen', 'object{a: int}', 'mixed', 'never', 'void', 'int|string', 'int|null',
        '__benevolent<int|string>', 'Countable&Traversable', '(int is int ? 1 : 2)', 'T', 'U', 'V', '*NEVER*', 'class-string<Countable>',
        'class-string<stdClass>', "'Countable'", "'Nope'", '__stringAndStringable', '__stringNotStringable', 'DateTimeInterface', 'DateTime|DateTimeImmutable',
        'Throwable', 'Error|Exception', 'array<int>', 'callable&array', '(T&Countable)', 'array{a: lowercase-string, b: int}', "1|array{}",
        'array{ArrayIterator<int, string>, ArrayIterator<int, int>}', 'array{a?: int}', 'array<int, int>',
        "'strlen'", "'ArrayIterator::count'", "'ArrayIterator::nope'", "'Closure::nope'", "'Nope::nope'", "''", "'1a::b'",
        "array{'ArrayIterator', 'count'}", "array{'ArrayIterator', 'nope'}", "array{'Closure', 'nope'}", "array{'Closure', 'bind'|'nope'}",
        "array{'Nope', 'x'}", "array{0: 'ArrayIterator', 1?: 'count'}", "array{'ArrayIterator', string}", "array{ArrayIterator, 'count'}", "array{1, 'count'}"];
    $sets = [[$atoms, 'ArrayIterator'], [['static', '$this', 'ArrayIterator', 'ArrayIterator<int, string>', 'Iterator', 'object', 'stdClass', 'Nope', 'Countable'], 'ArrayIterator'],
        [['static', '$this', 'Nope'], 'Nope']];
    $rows = [];
    $casts = [];
    $error = fn (Type $t) => $t instanceof PHPStan\Type\ErrorType;
    foreach ($sets as $i => [$set, $ctx]) {
        $scope = new NameScope(null, [], $ctx, null, $templates);
        foreach ($set as $a) {
            // `*NEVER*` is Pun's NeverType (`never` resolves to NonAcceptingNeverType)
            $resolve = fn (string $t) => $t === '*NEVER*' ? new PHPStan\Type\NeverType() : $resolver->resolve($t, $scope);
            $at = $resolve($a);
            if ($i === 0) {
                $casts[] = json_encode([$ctx, $pun($a), $error($at->toString()), $error($at->toNumber()), $error($at->toInteger()), $error($at->toFloat()), $error($at->toBitwiseNotType()), $at->isCloneable()->describe()], $flags);
            }
            foreach ([...$set, null, 'strict-mixed'] as $b) {
                $bt = $b === null ? null : ($b === 'strict-mixed' ? new PHPStan\Type\StrictMixedType() : $resolve($b));
                $rows[] = json_encode([$ctx, $pun($a), $b === null ? null : $pun($b),
                    $bt === null ? null : $at->accepts($bt, true)->result->describe(),
                    $bt === null ? null : $at->accepts($bt, false)->result->describe(),
                    $level(VerbosityLevel::getRecommendedLevelByType($at, $bt))], $flags);
            }
        }
    }
    $sm = new PHPStan\Type\StrictMixedType();
    $casts[] = json_encode([null, 'strict-mixed', $error($sm->toString()), $error($sm->toNumber()), $error($sm->toInteger()), $error($sm->toFloat()), $error($sm->toBitwiseNotType()), null], $flags);
    file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/type-accepts.json', "[\n".implode(",\n", $rows)."\n]\n");
    file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/type-casts.json', "[\n".implode(",\n", $casts)."\n]\n");
    echo count($rows), " accepts pairs, ", count($casts), " casts\n";
    exit;
}

if (($argv[1] ?? '') === 'probes') {
    $probes = array_map(fn ($s) => [null, $s], PROBES);
    foreach (['static|Countable', '$this|ArrayIterator', 'ArrayIterator|static', 'static|ArrayIterator', 'ArrayIterator|$this', '$this|static', 'static|$this', 'static&Countable', 'callable(static, $this): void', 'callable(): static', 'class-string<static>', 'static|SeekableIterator', 'SeekableIterator|static', 'Iterator&static', 'static&Iterator', 'stdClass|static', 'static|stdClass', 'static', '$this', 'self', 'static::class', 'self::class', 'ArrayIterator::class', 'self::STD_PROP_LIST', 'ArrayIterator::*', 'ArrayIterator::STD*LIST', 'ArrayIterator::*_PROP_*', 'ArrayIterator::*X*Y', 'ArrayIterator::S*P*T', 'ArrayIterator::*P*O*', 'list<static>', 'Countable::FOO', 'Nope::FOO'] as $s) {
        $probes[] = ['ArrayIterator', $s];
    }
    // every ordered pair of these atoms as `A|B` and `A&B`: the union/intersection normalization and isSuperTypeOf()
    // paths in both directions
    $atoms = ['int', '1', 'int<1, 5>', 'float', '1.5', 'bool', 'true', 'string', "'a'", "'Countable'", 'non-empty-string',
        'numeric-string', 'lowercase-string', 'uppercase-string', 'literal-string', 'class-string', 'class-string<Countable>',
        'null', 'resource', 'object', 'stdClass', 'Countable', 'Traversable<int>', 'Closure', 'Closure(int): void', 'callable',
        'callable(int): void', 'array', 'array{}', 'array{a: int}', 'list<int>', 'non-empty-array<int>', 'iterable<int>', 'static',
        '$this', 'Random\\IntervalBoundary::ClosedOpen', 'object{a: int}', 'mixed', 'never', 'void'];
    $strings = ['string', 'non-empty-string', 'non-falsy-string', 'numeric-string', 'lowercase-string', 'uppercase-string', 'literal-string',
        "''", "'0'", "'a'", "'A'", "'1'", 'class-string', 'callable-string', 'non-empty-lowercase-string', '(non-falsy-string&numeric-string)',
        '(uppercase-string&non-empty-string)', '(lowercase-string&non-falsy-string)'];
    $arrays = ['array', 'list<int>', 'non-empty-list<int>', 'array<string, int>', 'non-empty-array<int>', 'array{}', 'array{a: int}', 'array{a?: int}',
        'array{int, string}', 'list{int}', 'array{0?: int}', 'callable-array', 'iterable<int>', 'array<int, string>', 'array{a: 1, b: 2}', 'array{b: 2, a: 1}'];
    $ints = ['int', '0', '1', '-1', 'int<1, 5>', 'int<min, 0>', 'int<0, max>', 'positive-int', 'negative-int', '(1|2)', 'float', '1.0', 'int<2, 3>'];
    foreach ([[$atoms, 'ArrayIterator'], [$strings, null], [$arrays, null], [$ints, null]] as [$set, $ctx]) {
        foreach ($set as $a) {
            foreach ($set as $b) {
                if ($a !== $b) {
                    array_push($probes, [$ctx, "$a|$b"], [$ctx, "$a&$b"]);
                }
            }
        }
    }
    foreach (['T', 'U', 'V', 'T|never', 'never&T', 'T|null', 'T|U', 'U|T', 'T&U', 'array<U, T>', 'U[]', 'list<T>', 'iterable<T>|array<T>',
        'T&array', '(T is int ? string : bool)', '(T is int ? string : bool)&int', '(U is int ? string : bool)|string', 'class-string<V>',
        'callable(T): U', 'array{a: T}|array{a: U}', 'U|string|float', '(U&int)|string', 'V|stdClass', 'U&(int|bool)', 'array<int|iterable<int>>|array<iterable<int>>',
        'array<int|string>|array<int<1, 3>>', 'array<1|2|3>|array<int<1, 3>>', 'array<int<1, 3>>|array<1|2|3>', 'array<int<1, 3>>|array<1|2>'] as $s) {
        $probes[] = ['templates', $s];
    }
    foreach (['T', 'U', 'V', '(U&int)'] as $a) {
        foreach ($atoms as $b) {
            array_push($probes, ['templates', "$a|$b"], ['templates', "$a&$b"], ['templates', "$b|$a"], ['templates', "$b&$a"]);
        }
    }
    // sortTypes() leaves more than 1024 types unsorted
    $probes[] = [null, implode('|', array_map(fn ($i) => "'s$i'", range(1030, 1)))];
    // more than 63 constant-array keys overflow processArrayTypes()' bit set
    $keys = fn (string $v) => 'array{'.implode(', ', array_map(fn ($i) => "k$i: $v", range(0, 63))).'}';
    $probes[] = [null, $keys('1').'|'.$keys('2')];
    $probes[] = [null, 'array{}|'.$keys('1').'|'.$keys('2')];
    // reduceArrays() merging into an array that a later pair also names
    $probes[] = [null, 'array{a: 1, b: 1}|array{a: 1, c: 1}|array{a: 1}'];
    $probes[] = [null, 'array{a: 1, b: 1}|array{a: 1}|array{a: 1, c: 1}'];
    $probes[] = [null, 'Countable&(int&string)'];
    $probes[] = [null, 'iterable<int|string>&iterable<int|bool>'];
    $probes[] = ['templates', '(T&array)|array{a: 1}'];
    $probes[] = ['templates', '(T&list<int>)|list<int>'];
    // parser corners
    foreach (['($x int ? 1 : 2)', 'callable(): ?int', 'callable(): $this', 'callable(): DateTime::ATOM', 'callable(): (int)[', 'callable(): DateTime::',
        'callable(): DateTime::A* |int', '"\\e"', '"\\$"', '"\\u{}"', '"\\q"', 'EmptyIterator<int, string, bool>'] as $s) {
        $probes[] = [null, $s];
    }
    foreach (['decimal-int-string', 'non-decimal-int-string', 'ArrayIterator<int, covariant string>', 'ArrayIterator<contravariant int, string>'] as $s) {
        $probes[] = [null, $s];
    }
    $probes[] = ['ArrayIterator', 'parent'];
    $probes[] = ['RecursiveArrayIterator', 'parent'];
    $probes[] = ['Nope', 'static'];
    $probes[] = ['Nope', '$this'];
    $probes[] = ['DatePeriod', '$this'];
    // isSuperTypeOf() of every ordered pair within each atom set, one row per line: [context, a, b, result]
    $resolver = container(80400)->getByType(TypeStringResolver::class);
    $templates = probe_templates($resolver);
    $pun = fn (string $s) => preg_replace_callback('~\\b[TUV]\\b~', fn ($m) => sprintf("__template<%s, 'function probe()', %s>", $m[0], PROBE_TEMPLATES[$m[0]] ?? 'mixed'), $s);
    $rows = [];
    foreach ([[$atoms, 'ArrayIterator'], [$strings, null], [$arrays, null], [$ints, null], [['T', 'U', 'V', '(U&int)', 'int', 'string', "'a'", '1', 'Countable', 'Traversable<int>', 'array', 'mixed', 'never', 'object', 'null', 'int|string', 'Countable|int',
        "'Countable'", 'class-string<V>', '(T is int ? 1 : 2)', '(T is int ? 1 : string)', '__benevolent<int|string>'], 'templates'], [['true', 'false', 'bool', '1.5', '2.5', 'float', 'null', 'void', 'WeakMap', 'static', '$this', 'Closure', 'Closure(int): void', 'callable', 'object', 'stdClass', 'resource',
        'object{a: int}', 'array{}', 'iterable<int>', 'Countable', "'Countable'", "'a'", 'class-string<Countable>', 'class-string<static>', 'class-string'], 'WeakMap'],
        [['static', '$this', 'Closure', 'object', 'Countable'], 'Closure'],
        [['callable', 'callable(): void', 'callable(int): void', 'callable(int=): void', 'callable(int...): void', 'callable(int, string): void',
            'callable(int, string=): int', 'callable(int=, string...): void', 'Closure(int): void', 'Closure(int...): void', 'Closure', 'callable(mixed): mixed',
            'callable(string): void', "'strlen'", 'callable-string', 'array', 'Foo', 'Bar', 'ArrayIterator', 'ArrayIterator<int, string>', 'ArrayIterator<string, int>',
            'Iterator<int, string>', 'Traversable<int, string>', 'SplObjectStorage<*, *>', 'SplObjectStorage<Countable, int>', 'Generator<int, string>', 'Iterator',
            'IteratorAggregate<int, string>', 'ArrayObject<int, string>', 'WeakMap<Countable, int>', '1|2|3', 'int<1, 3>', '(1|2)', 'int<1, 2>'], null],
        [['(int is int ? 1 : 2)', '(int is not int ? 1 : 2)', '1', '2', 'SplObjectStorage<Countable>', 'SplObjectStorage<Countable, int>',
            'ArrayIterator<array-key, int>', 'ArrayIterator<int, int>', 'ArrayAccess<int>', 'ArrayAccess<int, string>'], null],
        [['decimal-int-string', 'non-decimal-int-string', "'12'", "'012'", "''", 'numeric-string', 'lowercase-string', 'uppercase-string',
            'non-empty-string', 'string', 'int', 'callable-string', 'decimal-int-string&non-falsy-string'], null]] as [$set, $ctx]) {
        $scope = $ctx === 'templates' ? new NameScope(null, [], null, null, $templates) : new NameScope(null, [], $ctx);
        foreach ($set as $a) {
            foreach ($set as $b) {
                $result = $resolver->resolve($a, $scope)->isSuperTypeOf($resolver->resolve($b, $scope))->result->describe();
                $rows[] = json_encode($ctx === 'templates' ? [null, $pun($a), $pun($b), $result] : [$ctx, $a, $b, $result], $flags);
            }
        }
    }
    file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/type-super.json', "[\n".implode(",\n", $rows)."\n]\n");
    // TypehintHelper::decideType(native, phpdoc): [native, phpdoc or null, described]
    $decide = [];
    $plain = new NameScope(null, []);
    $withTemplates = new NameScope(null, [], null, null, $templates);
    foreach ([['mixed', null], ['int', null], ['mixed', 'int'], ['int', 'positive-int'], ['?int', 'positive-int'], ['int', 'string'],
        ['?int', 'string|null'], ['int', 'int|null'], ['?int', 'int|null'], ['mixed', 'void'], ['void', 'never'], ['mixed', 'never'], ['int', 'never'],
        ['array', 'array<int, string>'], ['array', 'iterable<string>'], ['iterable', 'array<string>'], ['?array', 'list<int>|null'],
        ['int|string', 'int'], ['__benevolent<int|string>', 'int'], ['mixed', 'T'], ['int', 'U'], ['?Countable', 'V'], ['?int', 'T'],
        ['array', 'T'], ['int|null', 'int'], ['string', "'a'|'b'"], ['?string', "'a'|null"], ['Traversable', 'iterable<int>'],
        ['iterable', 'Traversable<int>'], ['object', 'Countable'], ['Countable', 'stdClass'], [null, 'void'], [null, 'int'], [null, null], ['null', 'int'], ['null', 'null'],
        ['iterable', 'array<string>|Countable'], ['?iterable', 'array<int>|null']] as [$native, $phpdoc]) {
        // no native type: an implicit mixed
        $nativeType = $native === null ? new PHPStan\Type\MixedType() : $resolver->resolve($native, $plain);
        $phpdocType = $phpdoc === null ? null : $resolver->resolve($phpdoc, $withTemplates);
        $decide[] = [$native === null ? null : $pun($native), $phpdoc === null ? null : $pun($phpdoc), PHPStan\Type\TypehintHelper::decideType($nativeType, $phpdocType)->describe(VerbosityLevel::precise())];
    }
    file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/type-decide.json', json_encode($decide, $flags | JSON_PRETTY_PRINT)."\n");
    // type strings in a project namespace and class (tests/fixtures/scope-classes.php): [class or null, type, described]
    require $root.'/crates/pun-phpstan/tests/fixtures/scope-classes.php';
    $scoped = [];
    foreach (['PunProbe\\Child', 'PunProbe\\Coll', 'PunProbe\\Missing', null] as $class) {
        $scope = new NameScope('PunProbe', [], $class);
        foreach (['static', 'static<string>', 'self', 'parent', 'Coll', 'Coll<int, string>', 'Child<string>', 'SORT_REGULAR',
            'int-mask-of<SORT_REGULAR|SORT_NUMERIC>', '\\SORT_NUMERIC', 'self::A', 'self::*', 'Child::C', 'Coll::*', 'class-string<static>',
            'Nope::A', 'Coll::NOPE', 'Defaults<string>', 'Defaults<string, string, string, string>', 'value-of<Backed>',
            'value-of<One>', 'value-of<Suit>', 'value-of<Backed::A>', 'key-of<Backed>', 'key-of<array<int, string>>',
            'value-of<list<string>>', 'value-of<int>', 'value-of<int, int>', 'pure-callable(): void',
            'callable<X of self>(X, parent, SORT_REGULAR): static', 'Coll<covariant int, contravariant string>', 'Coll<*, int>',
            'static<covariant string>', 'array<int, covariant string>', 'list<contravariant int>', "array{a: int, b?: string}['a']",
            "array{a: int, b?: string}['b']", "array{a: int}['z']", "array<string, int>['k']", "array<int, string>['k']", 'list<string>[0]',
            "int['k']", 'string[0]', "string['k']", "mixed['k']", "(array{a: int}|array{a: string})['a']", "(array{a: int}|int)['a']",
            "array{a: int, ...<string, bool>}['z']", "non-empty-array<string, int>['k']", "Coll['k']", "\\ArrayObject['k']",
            "array{a: int}[string]", 'non-empty-list<string>[0]', "Coll<covariant int, string>['k']"] as $s) {
            $scoped[] = [$class, $s, $resolver->resolve($s, $scope)->describe(VerbosityLevel::precise())];
        }
    }
    $scope = new NameScope('PunProbe', [], 'PunProbe\\Child', null, $templates);
    foreach (['T', 'list<U>', 'Coll<T, V>', 'V<int>', 'U<int>', 'value-of<T>', 'callable<X>(X, T): U',
        'callable<X of Coll = Child<int>, Y>(X): Y', "T['k']", 'list<U>[0]', "(T|U)['k']"] as $s) {
        $scoped[] = ['templates', $s, $resolver->resolve($s, $scope)->describe(VerbosityLevel::precise())];
    }
    // a KeyOfType / ValueOfType of a template resolved late (`resolve()`), over templates E of BackedEnum, F of
    // PunProbe\Backed and G of array<int, string>
    $lateScope = PHPStan\Type\Generic\TemplateTypeScope::createWithFunction('probe');
    $late = [];
    foreach (['E' => 'BackedEnum', 'F' => 'PunProbe\\Backed', 'G' => 'array<int, string>'] as $name => $bound) {
        $late[$name] = PHPStan\Type\Generic\TemplateTypeFactory::create($lateScope, $name, $resolver->resolve($bound, new NameScope(null, [])), PHPStan\Type\Generic\TemplateTypeVariance::createInvariant());
    }
    $scope = new NameScope('PunProbe', [], null, null, new PHPStan\Type\Generic\TemplateTypeMap($late));
    foreach (['value-of<E>', 'key-of<E>', 'value-of<F>', 'key-of<G>', 'value-of<G>'] as $s) {
        $t = $resolver->resolve($s, $scope);
        if ($t instanceof PHPStan\Type\LateResolvableType) {
            $t = $t->resolve();
        }
        $scoped[] = ['late', $s, $t->describe(VerbosityLevel::precise())];
    }
    // OffsetAccessTypes resolved late (`TypeUtils::resolveLateResolvableTypes()`)
    $scope = new NameScope('PunProbe', []);
    foreach (["array{a: int, b?: string}['a']", "array{a: int, b?: string}['b']", "array<string, int>['k']", 'list<string>[0]', 'string[0]',
        "mixed['k']", "(array{a: int}|array{a: string})['a']", "(array{a: int}|int)['a']", "array{a: int, ...<string, bool>}['z']",
        "non-empty-array<string, int>['k']", "array{a: list<int>}['a'][0]"] as $s) {
        $scoped[] = ['resolved', $s, PHPStan\Type\TypeUtils::resolveLateResolvableTypes($resolver->resolve($s, $scope))->describe(VerbosityLevel::precise())];
    }
    file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/type-scope.json', json_encode($scoped, $flags | JSON_PRETTY_PRINT)."\n");
    // isSuperTypeOf() between the project classes: [a, b, result]
    $scope = new NameScope('PunProbe', []);
    $projectSuper = [];
    foreach ([['Coll<int, string>', 'Child<string>'], ['Coll<int, string>', 'Child<int>'], ['Coll', 'Child<string>'], ['Child<string>', 'Coll<int, string>'],
        ['Coll<string, string>', 'Child<string>'], ['Child', 'Child<int>'], ['Coll<int, string>', 'Plain'], ['Coll', 'Plain'], ['Coll', 'Tr'],
        ['Tr', 'Coll'], ['Tr', 'Tr'], ['Tr&\\stdClass', 'Tr'], ['Coll<covariant \\Countable, int>', 'Coll<\\ArrayIterator, int>'],
        ['Coll<covariant \\Countable, int>', 'Coll<int, int>'], ['Coll<contravariant \\ArrayIterator, int>', 'Coll<\\Countable, int>'],
        ['Coll<contravariant \\Countable, int>', 'Coll<\\ArrayIterator, int>'], ['Coll<int, int>', 'Coll<covariant int, int>'],
        ['Coll<covariant int, int>', 'Coll<covariant int, int>'], ['Coll<covariant int, int>', 'Coll<contravariant int, int>'],
        ['Coll<covariant mixed, int>', 'Coll<string, int>'], ['Coll<covariant int, int>', 'Coll<never, int>'], ['Coll<covariant int, int>', 'Coll<*, int>']] as [$a, $b]) {
        $projectSuper[] = [$a, $b, $resolver->resolve($a, $scope)->isSuperTypeOf($resolver->resolve($b, $scope))->result->describe()];
    }
    file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/type-project-super.json', json_encode($projectSuper, $flags | JSON_PRETTY_PRINT)."\n");
    // ConstExprNodeResolver::resolve() (a `@method` parameter's default value): [class or null, expression, described]
    $constParser = new PHPStan\PhpDocParser\Parser\ConstExprParser(new PHPStan\PhpDocParser\ParserConfig([]));
    $lexer = new PHPStan\PhpDocParser\Lexer\Lexer(new PHPStan\PhpDocParser\ParserConfig([]));
    $constResolver = container(80400)->getByType(PHPStan\PhpDoc\ConstExprNodeResolver::class);
    $constants = [];
    foreach (['PunProbe\\Child', 'PunProbe\\Coll', null] as $class) {
        foreach (["[1, 'a' => true]", 'false', 'true', 'null', '1', '1.5', "'x'", 'self::A', 'parent::A', 'static::C', 'Coll::B',
            'Child::A', 'Nope::A', 'Coll::NOPE', 'FOO', 'Coll::class', 'Suit::Hearts', 'Suit::X'] as $s) {
            $node = $constParser->parse(new PHPStan\PhpDocParser\Parser\TokenIterator($lexer->tokenize($s)));
            $constants[] = [$class, $s, $constResolver->resolve($node, new NameScope('PunProbe', [], $class))->describe(VerbosityLevel::precise())];
        }
    }
    file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/type-const-expr.json', json_encode($constants, $flags | JSON_PRETTY_PRINT)."\n");
    // the typeAliases parameter's global type aliases: [aliases, type, described in namespace PunProbe]
    $globalAliases = ['PunAlias' => 'int|string', 'PunNested' => 'list<PunAlias>', 'PunClass' => 'Countable'];
    $tmp = sys_get_temp_dir().'/pun-symbols-aliases';
    @mkdir($tmp);
    file_put_contents($tmp.'/v.neon', "parameters:\n\tphpVersion: 80400\n\ttypeAliases:\n".implode('', array_map(fn ($k, $v) => "\t\t$k: '$v'\n", array_keys($globalAliases), $globalAliases)));
    $aliasResolver = (new ContainerFactory($tmp))->create($tmp, [$tmp.'/v.neon'], [])->getByType(TypeStringResolver::class);
    $aliased = [];
    foreach (['PunAlias', 'PunNested', 'array<PunAlias, PunClass>', 'punalias'] as $s) {
        $aliased[] = [$globalAliases, $s, $aliasResolver->resolve($s, new NameScope('PunProbe', []))->describe(VerbosityLevel::precise())];
    }
    file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/type-global-aliases.json', json_encode($aliased, $flags | JSON_PRETTY_PRINT)."\n");
    // getObjectClassNames(): [type, names]
    $names = [];
    foreach (['Coll|Child<int>|int', 'Coll&\\Countable', 'Coll|Coll<int, int>'] as $t) {
        $names[] = [$t, $resolver->resolve($t, $scope)->getObjectClassNames()];
    }
    file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/type-object-class-names.json', json_encode($names, $flags | JSON_PRETTY_PRINT)."\n");
    // getIterableKeyType() / getIterableValueType() and getAncestorWithClassName(): [type, key, value, Coll ancestor]
    $project = [];
    foreach (['Child<string>', 'Nope<int>', 'Coll<int, string>', '\\ArrayIterator<int, covariant string>'] as $t) {
        $type = $resolver->resolve($t, $scope);
        $ancestor = $type->getAncestorWithClassName('PunProbe\\Coll');
        $project[] = [$t, $type->getIterableKeyType()->describe(VerbosityLevel::precise()), $type->getIterableValueType()->describe(VerbosityLevel::precise()), $ancestor?->describe(VerbosityLevel::precise())];
    }
    file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/type-project.json', json_encode($project, $flags | JSON_PRETTY_PRINT)."\n");
    // TypeTraverser::map() replacing int with string: [type, mapped]
    $traversed = [];
    foreach (['array{a: int, b?: list<int>}', 'array{int, ...<int, int>}', 'list<int>|int', 'Coll<int, string>', 'class-string<Coll<int, int>>', '(T is int ? int : string)', 'static<int>',
        'Coll<covariant int, string>', "array{a: int}['a']"] as $t) {
        $type = $resolver->resolve($t, new NameScope('PunProbe', [], 'PunProbe\\Child', null, $templates));
        $mapped = PHPStan\Type\TypeTraverser::map($type, function (PHPStan\Type\Type $t, callable $traverse) {
            return $t instanceof PHPStan\Type\IntegerType ? new PHPStan\Type\StringType() : $traverse($t);
        });
        $traversed[] = [$pun($t), $mapped->describe(VerbosityLevel::precise())];
    }
    file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/type-traverse.json', json_encode($traversed, $flags | JSON_PRETTY_PRINT)."\n");
    // TypeCombinator::addNull() and removeNull(): [type, added, removed]
    $nulls = [];
    foreach (['int', 'int|null', 'null', '__benevolent<int|null>', '__benevolent<int|string|null>', 'mixed', 'never', 'void', 'int|string', 'T'] as $t) {
        $type = $resolver->resolve($t, $withTemplates);
        $nulls[] = [$pun($t), PHPStan\Type\TypeCombinator::addNull($type)->describe(VerbosityLevel::precise()), PHPStan\Type\TypeCombinator::removeNull($type)->describe(VerbosityLevel::precise())];
    }
    file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/type-null.json', json_encode($nulls, $flags | JSON_PRETTY_PRINT)."\n");
    file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/type-probes.json', json_encode(describe_all(container(80400)->getByType(TypeStringResolver::class), $probes), $flags | JSON_PRETTY_PRINT)."\n");
    echo count($probes), " probes, ", count($rows), " isSuperTypeOf pairs\n";
    exit;
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

@mkdir($root.'/crates/pun-phpstan/data');
file_put_contents($root.'/crates/pun-phpstan/data/symbols.json', json_encode($symbols, $flags)."\n");
file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/type-strings.json', json_encode($described, $flags | JSON_PRETTY_PRINT)."\n");
file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/reflected-types.json', json_encode($reflectedList, $flags | JSON_PRETTY_PRINT)."\n");
echo implode(', ', array_map(fn ($k) => count($v84[$k])." $k (".count($delta[$k] ?? []).' differ on 8.5)', array_keys($v84))), ', ', count($described), ' signature-map type strings, ', count($reflectedList), " reflected types\n";
