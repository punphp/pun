<?php
namespace GenericDefaults;

/** @template T of int|string = int */
class NoConstructor
{
    /** @var T */
    public $value;
}

/** @template T of int = 42 */
class EmptyConstructor
{
    /** @var T */
    public $value;
    public function __construct() {}
}

/** @template T of int = 42 */
class LiteralDefault
{
    /** @var T */
    public $value;
}

/**
 * @template Z = string
 * @template A = list<Z>
 * @template M = array<string, A>
 */
class OrderedDefaults {}

/**
 * @template Z = string
 * @template A = list<Z>
 * @template M = array<string, A>
 */
class OrderedEmptyConstructor
{
    /** @var M */
    public $nested;
    public function __construct() {}
}

/**
 * @template T
 * @template U = list<T>
 */
class DependentConstructor
{
    /** @var U */
    public $items;
    /** @param T $first */
    public function __construct(public mixed $first) {}
}

/** @template T of int|string = int */
class InferredDefault
{
    /** @param T $value */
    public function __construct(public int|string $value) {}
}

/** @template T */
class ConstructorParent
{
    /** @param T $value */
    public function __construct(public mixed $value) {}
    /** @return T */
    public function getValue(): mixed { return $this->value; }
}

/**
 * @template T = string
 * @template U = list<T>
 * @extends ConstructorParent<T>
 */
class InheritedDefaults extends ConstructorParent
{
    /** @var U */
    public $items;
}

/**
 * @template TId of int|null = int|null
 * @extends ConstructorParent<string>
 */
class InheritedScalarDefault extends ConstructorParent
{
    /** @var TId */
    public $id;
}

/**
 * @template T
 * @template U
 * @extends ConstructorParent<T>
 */
class InheritedRequired extends ConstructorParent {}

/**
 * @template T of object
 * @template U = int
 */
class PartiallyDefaulted
{
    /** @var U */
    public $value;
}

/** @template T of int|null = int|null */
class NullableDefault
{
    /** @var T */
    public $value;
}

/** @template T of int = self::VALUE */
class SelfConstant
{
    public const VALUE = 42;
    /** @var T */
    public $value;
}

/** @template T of object = \stdClass */
class ObjectDefault {}

/** @param 42 $value */
function needs42(int $value): void {}
/** @param 43 $value */
function needs43(int $value): void {}
function needsInt(int $value): void {}
function needsString(string $value): void {}

/**
 * @template T
 * @param callable(): T $callback
 * @return T
 */
function callbackValue(callable $callback): mixed { return $callback(); }

function defaults(string $text, int $integer): void
{
    $plain = new NoConstructor();
    \PHPStan\dumpType($plain);
    \PHPStan\dumpType($plain->value);
    needsInt($plain->value);
    needsString($plain->value);
    $empty = new EmptyConstructor();
    \PHPStan\dumpType($empty);
    \PHPStan\dumpType($empty->value);
    needs42($empty->value);
    needs43($empty->value);
    $literal = new LiteralDefault();
    \PHPStan\dumpType($literal);
    needs42($literal->value);
    needs43($literal->value);
    \PHPStan\dumpType(new OrderedDefaults());
    $ordered = new OrderedEmptyConstructor();
    \PHPStan\dumpType($ordered->nested);
    $dependent = new DependentConstructor($text);
    \PHPStan\dumpType($dependent->items);
    \PHPStan\dumpType(new InferredDefault($text));
    \PHPStan\dumpType(new InferredDefault($integer));
    $strings = new InheritedDefaults($text);
    \PHPStan\dumpType($strings->items);
    \PHPStan\dumpType($strings->getValue());
    $integers = new InheritedDefaults($integer);
    \PHPStan\dumpType($integers->items);
    \PHPStan\dumpType(new InheritedRequired($text));
    $partial = new PartiallyDefaulted();
    \PHPStan\dumpType($partial->value);
    $nullable = new NullableDefault();
    \PHPStan\dumpType($nullable);
    \PHPStan\dumpType($nullable->value);
    needsInt($nullable->value);
    $inheritedScalar = new InheritedScalarDefault($text);
    \PHPStan\dumpType($inheritedScalar);
    \PHPStan\dumpType($inheritedScalar->id);
    needsInt($inheritedScalar->id);
    $constant = new SelfConstant();
    \PHPStan\dumpType($constant);
    \PHPStan\dumpType($constant->value);
    needs42($constant->value);
    needs43($constant->value);
    \PHPStan\dumpType(new ObjectDefault());
    \PHPStan\dumpType(callbackValue(static fn () => new NoConstructor()));
}
