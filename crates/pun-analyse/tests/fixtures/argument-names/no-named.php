<?php declare(strict_types=1);

namespace NoNamedArguments;

/** @no-named-arguments */
function forbidden(int $value = 0): int { echo $value; return $value; }
function allowed(int $value = 0): int { echo $value; return $value; }

/** @no-named-arguments */
function variadic(int ...$values): void {}

/** @no-named-arguments */
function noParameters(): void {}

/** @no-named-arguments */
function bodyVariadic(int $value): void { echo count(func_get_args()); }

forbidden(value: 1);
forbidden(value: 'wrong');
forbidden(unknown: 1);
forbidden(1, value: 2);
forbidden(1, 2, value: 3);
forbidden(1);
allowed(value: 1);
variadic(extra: 1, values: 2);
noParameters(unknown: 1);
bodyVariadic(value: 1);
bodyVariadic(1, extra: 2);

forbidden(...['value' => 1]);
forbidden(...[0 => 1]);
forbidden(...['0' => 1]);
forbidden(...[]);
variadic(...['first' => 1, 'second' => 2]);
allowed(...['value' => 1]);

/**
 * @param array<string, int> $strings
 * @param array<array-key, int> $possibly
 * @param list<int> $integers
 * @param array{value: int} $shape
 * @param iterable<string, int> $iterable
 */
function unpack(array $strings, array $possibly, array $integers, array $shape, iterable $iterable): void
{
    forbidden(...$strings);
    forbidden(...$possibly);
    forbidden(...$integers);
    forbidden(...$shape);
    forbidden(...$iterable);
    $constant = ['value' => 1];
    forbidden(...$constant);
    allowed(...$strings);
}

class Methods
{
    /** @no-named-arguments */
    public function instance(int $value = 0): void {}
    /** @no-named-arguments */
    public static function staticMethod(int $value = 0): void {}
    /** @no-named-arguments */
    public function __construct(int $value = 0) {}
}

$object = new Methods(value: 1);
$object->instance(value: 1);
Methods::staticMethod(value: 1);

/** @no-named-arguments */
class TaggedClass
{
    public function __construct(int $value = 0) {}
    public function instance(int $value = 0): void {}
    public static function staticMethod(int $value = 0): void {}
    public function __invoke(int $value = 0): void {}
}

$tagged = new TaggedClass(value: 1);
$tagged->instance(value: 1);
TaggedClass::staticMethod(value: 1);
$tagged(value: 1);

class Child extends Methods
{
    public function instance(int $value = 0): void {}
}
$child = new Child(value: 1);
$child->instance(value: 1);

// The parent's class tag is not inherited by an overriding method's PHPDoc.
class UntaggedChild extends TaggedClass
{
    public function instance(int $value = 0): void {}
}
(new UntaggedChild())->instance(value: 1);

interface Contract
{
    /** @no-named-arguments */
    public function inherited(int $value): void;
}
class Implementation implements Contract
{
    public function inherited(int $value): void {}
}
(new Implementation())->inherited(value: 1);

#[\Attribute(\Attribute::TARGET_ALL | \Attribute::IS_REPEATABLE)]
class AttributeConstructor
{
    /** @no-named-arguments */
    public function __construct(int $value = 0) {}
}
/** @no-named-arguments */
#[\Attribute]
class AttributeClass
{
    public function __construct(int $value = 0) {}
}
#[\Attribute]
class InheritedAttribute extends AttributeConstructor {}

#[AttributeConstructor(value: 1)]
#[AttributeConstructor(value: 'wrong')]
#[AttributeConstructor(unknown: 1)]
#[AttributeConstructor(1)]
#[AttributeClass(value: 1)]
#[InheritedAttribute(value: 1)]
class Attributed {}

$function = __NAMESPACE__ . '\\forbidden';
$function(value: 1);
$method = [$object, 'instance'];
$method(value: 1);
$static = Methods::class . '::staticMethod';
$static(value: 1);
$result = call_user_func(__NAMESPACE__ . '\\forbidden', value: 1);
$result = call_user_func_array(__NAMESPACE__ . '\\forbidden', ['value' => 1]);
$result = call_user_func_array(__NAMESPACE__ . '\\forbidden', [1]);

/** @param callable(int $value): void $callback */
function typedCallable(callable $callback): void { $callback(value: 1); }
$closure = function (int $value): void { echo $value; };
$closure(value: 1);

// A callable union retains every callee's policy even when signature selection chooses the permissive one.
function unionCallable(bool $flag): void
{
    $callback = $flag ? __NAMESPACE__ . '\\forbidden' : __NAMESPACE__ . '\\allowed';
    $callback(value: 1);
}

// Invoked first-class method callables retain the overriding method's inherited PHPDoc policy.
$callback = $child->instance(...);
$callback(value: 1);
$callback(1);
// A parent's class-only tag still does not prohibit the child's overriding method callable.
$callback = (new UntaggedChild())->instance(...);
$callback(value: 1);
$callback(1);

trait CallableRequirements
{
    /** @no-named-arguments */
    abstract public function required(int $value = 0): void;
    abstract public function untagged(int $value = 0): void;
    /** @return void */
    abstract public function documented(int $value = 0): void;
    /** @no-named-arguments */
    public function concrete(int $value = 0): void {}
}
class CallableTraitImplementation
{
    use CallableRequirements;

    /** @return void */
    public function required(int $value = 0): void {}
    public function untagged(int $value = 0): void {}
    public function documented(int $value = 0): void {}
    public function concrete(int $value = 0): void {}
}
$requirements = new CallableTraitImplementation();
$callback = $requirements->required(...);
$callback(value: 1);
$callback(1);
// Neither an untagged abstract requirement nor an overridden concrete trait method forbids names.
$callback = $requirements->untagged(...);
$callback(value: 1);
$callback = $requirements->documented(...);
$callback(value: 1);
$callback = $requirements->concrete(...);
$callback(value: 1);

interface CallableRoot
{
    public function run(int $value = 0): void;
}
interface CallableLeft extends CallableRoot {}
interface CallableRight extends CallableRoot {}
interface CallableForbidden
{
    /** @no-named-arguments */
    public function run(int $value = 0): void;
}
class CallableDiamond implements CallableLeft, CallableRight, CallableForbidden
{
    public function run(int $value = 0): void {}
}
class CallableAllowedDiamond implements CallableLeft, CallableRight
{
    public function run(int $value = 0): void {}
}
// Revisiting Root through Right must not hide the later Forbidden interface's policy.
$callback = (new CallableDiamond())->run(...);
$callback(value: 1);
$callback(1);
$callback = (new CallableAllowedDiamond())->run(...);
$callback(value: 1);
$callback(1);
