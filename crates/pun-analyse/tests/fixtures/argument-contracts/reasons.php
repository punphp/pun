<?php declare(strict_types=1);

namespace ArgumentReasonOracle;

/**
 * @template K
 * @param K $key
 * @param array{K: int} $value
 */
function keyed($key, $value): void {}

/**
 * @template T of array
 * @param T $source
 * @param T&array{target: int} $value
 */
function intersected($source, $value): void {}

final class Methods
{
    /**
     * @template K
     * @param K $key
     * @param array{K: int} $value
     */
    public function keyed($key, $value): void {}

    /**
     * @template T of array
     * @param T $source
     * @param T&array{target: int} $value
     */
    public function intersected($source, $value): void {}
}

#[\Attribute(\Attribute::TARGET_CLASS)]
final class KeyedAttribute
{
    /**
     * @template K
     * @param K $key
     * @param array{K: int} $value
     */
    public function __construct($key, $value) {}
}

#[\Attribute(\Attribute::TARGET_CLASS)]
final class IntersectedAttribute
{
    /**
     * @template T of array
     * @param T $source
     * @param T&array{target: int} $value
     */
    public function __construct($source, $value) {}
}

keyed(new \stdClass(), []);
keyed(['a'], []);
keyed('valid', ['valid' => 1]);
intersected(['other' => 1], []);
intersected(['target' => 'wrong'], []);
intersected(['target' => 1], ['target' => 1]);
$methods = new Methods();
$methods->keyed(new \stdClass(), []);
$methods->intersected(['other' => 1], []);

#[KeyedAttribute(new \stdClass(), [])]
final class ObjectKey {}
#[KeyedAttribute(['a'], [])]
final class ArrayKey {}
#[IntersectedAttribute(['other' => 1], [])]
final class MissingOffset {}
#[IntersectedAttribute(['target' => 'wrong'], [])]
final class WrongOffset {}

/**
 * @template K
 * @template L
 * @param K $first
 * @param L $second
 * @param array{first: array{K: int}, second: array{L: int}, duplicate: array{K: int}} $value
 */
function ordered($first, $second, $value): void {}

/**
 * @template K
 * @template L
 * @param K $first
 * @param L $second
 * @param array{second: array{L: int}, first: array{K: int}, duplicate: array{L: int}} $value
 */
function reversed($first, $second, $value): void {}

/**
 * @template K
 * @template L
 * @param K $first
 * @param L $second
 * @param array{first: array{K: int}, container: array{second: array{L: int}}} $value
 */
function nested($first, $second, $value): void {}

/**
 * @template K
 * @param K $key
 * @param array{broken: value-of<int>, key: array{K: int}} $value
 */
function alreadyInvalid($key, $value): void {}

ordered(new \stdClass(), ['a'], ['first' => [], 'second' => [], 'duplicate' => []]);
reversed(new \stdClass(), ['a'], ['first' => [], 'second' => [], 'duplicate' => []]);
nested(new \stdClass(), ['a'], ['first' => [], 'container' => ['second' => []]]);
alreadyInvalid(new \stdClass(), ['broken' => 0, 'key' => []]);

/**
 * @template K
 * @param K $key
 * @param array{K: int}&array{target: int} $value
 */
function lateIntersection($key, $value): void {}

lateIntersection('other', []);
lateIntersection('target', []);

final class NoProperty {}
final class PrivateProperty { private int $target = 1; }
final class StaticProperty { public static int $target = 1; }

/**
 * @template T of array
 * @param T $source
 * @param T['shape']&object{target: int} $value
 */
function objectIntersection($source, $value): void {}

objectIntersection(['shape' => new NoProperty()], new NoProperty());
objectIntersection(['shape' => new PrivateProperty()], new PrivateProperty());
objectIntersection(['shape' => new StaticProperty()], new StaticProperty());

/**
 * @template T of array
 * @param T $source
 * @param T['callback']&callable(): void $value
 */
function callableIntersection($source, $value): void {}

callableIntersection(['callback' => static function (int $value): void {}], static function (): void {});

/** @template T */
final class Box {}

/**
 * @template T of array
 * @param T $source
 * @param T['box']&Box<int> $value
 */
function genericIntersection($source, $value): void {}

/** @param Box<1> $box */
function genericControl(Box $box): void
{
    genericIntersection(['box' => $box], $box);
}

/**
 * @template T of array
 * @param T $source
 * @param T['callback']&(\Closure(): void) $value
 */
function closureIntersection($source, $value): void {}

closureIntersection(['callback' => static function (int $value): void {}], static function (): void {});
closureIntersection(['callback' => static function (?int $value = null): void {}], static function (int $value): void {});

final class WriteOnlyProperty { public int $target { set {} } }
objectIntersection(['shape' => new WriteOnlyProperty()], new WriteOnlyProperty());

#[\Attribute(\Attribute::TARGET_CLASS)]
final class ObjectIntersectionAttribute
{
    /**
     * @template T of array
     * @param T $source
     * @param T['shape']&object{target: int} $value
     */
    public function __construct($source, $value) {}

    /**
     * @template T of array
     * @param T $source
     * @param T['shape']&object{target: int} $value
     */
    public function method($source, $value): void {}
}

#[ObjectIntersectionAttribute(['shape' => new NoProperty()], new NoProperty())]
final class MissingPropertyAttribute {}
#[ObjectIntersectionAttribute(['shape' => new PrivateProperty()], new PrivateProperty())]
final class PrivatePropertyAttribute {}

$object = new ObjectIntersectionAttribute(['shape' => new NoProperty()], new NoProperty());
$object->method(['shape' => new NoProperty()], new NoProperty());

/**
 * @template T of array
 * @param T $source
 * @param array{first: T['a']&object{target: int}, second: T['b']&object{target: int}, repeat: T['a']&object{target: int}} $value
 */
function neverOrdered($source, $value): void {}

neverOrdered(
    ['a' => new NoProperty(), 'b' => new PrivateProperty()],
    ['first' => new NoProperty(), 'second' => new PrivateProperty(), 'repeat' => new NoProperty()],
);

/**
 * @template T of array
 * @param T $source
 * @param array{broken: value-of<int>, value: T['shape']&object{target: int}} $value
 */
function alreadyInvalidNever($source, $value): void {}

alreadyInvalidNever(['shape' => new NoProperty()], ['broken' => 0, 'value' => new NoProperty()]);

final class PublicProperty { public int $target = 1; }
final class WrongPropertyType { public string $target = 'one'; }
objectIntersection(['shape' => new PublicProperty()], new PublicProperty());
objectIntersection(['shape' => new WrongPropertyType()], new WrongPropertyType());

/**
 * @template T of array
 * @param T $source
 * @param T['shape']&object{target?: int} $value
 */
function optionalProperty($source, $value): void {}

optionalProperty(['shape' => new NoProperty()], new NoProperty());

class ExtensibleNoProperty {}
#[\AllowDynamicProperties]
final class DynamicNoProperty {}
objectIntersection(['shape' => new ExtensibleNoProperty()], new ExtensibleNoProperty());
objectIntersection(['shape' => new DynamicNoProperty()], new DynamicNoProperty());
objectIntersection(['shape' => new \stdClass()], new \stdClass());
