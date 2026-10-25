<?php declare(strict_types=1);

namespace ArgumentContracts;

/**
 * @template T
 * @param T $input
 * @param value-of<T> $value
 */
function value($input, $value): int { return 1; }

/**
 * @template T
 * @param T $input
 * @param key-of<T> $key
 */
function key($input, $key): void {}

/**
 * @template T of array
 * @param T $input
 * @param T['required'] $value
 */
function offset(array $input, $value): void {}

#[\Attribute(\Attribute::TARGET_CLASS | \Attribute::IS_REPEATABLE)]
class GenericValue
{
    /**
     * @template T
     * @param T $input
     * @param value-of<T> $value
     */
    public function __construct($input, $value) {}

    /**
     * @template T
     * @param T $input
     * @param value-of<T> $value
     */
    public function method($input, $value): void {}

    /**
     * @template T
     * @param T $input
     * @param value-of<T> $value
     */
    public static function staticMethod($input, $value): void {}
}

#[GenericValue(0, 0)]
#[GenericValue('', 0)]
#[GenericValue(value: 0, input: 0)]
#[GenericValue([1], 0)]
#[GenericValue([1], 1)]
class GenericTagged {}

function valueCalls(GenericValue $object): void
{
    value(0, 0);
    value('', 0);
    value([1], 0);
    value([1], 1);
    value(value: 0, input: 0);
    value(...[0, 0]);
    key(0, 0);
    key(['x' => 1], 0);
    key(['x' => 1], 'x');
    offset([], 0);
    offset(['required' => 1], 0);
    offset(['required' => 1], 1);
    $object->method(0, 0);
    $object->method(value: 0, input: 0);
    GenericValue::staticMethod(0, 0);
    $instance = new GenericValue(0, 0);
    $instance = new GenericValue(value: 0, input: 0);
    $callable = __NAMESPACE__.'\\value';
    $callable(0, 0);
    $result = call_user_func(__NAMESPACE__.'\\value', 0, 0);
    $result = call_user_func_array(__NAMESPACE__.'\\value', [0, 0]);
    $first = value(...);
    $first(0, 0);
}

/**
 * @template T
 * @param T $input
 * @param array<value-of<T>> $values
 */
function nested($input, array $values): void {}

/** @param int&string $value */
function alreadyBroken($value): void {}

/** @param never $value */
function explicitNever($value): void {}

function controls(): void
{
    nested(0, []);
    nested([1], [1]);
    nested([1], ['wrong']);
    alreadyBroken(0);
    explicitNever(0);
}

/**
 * @template T
 * @param T $input
 * @param (T is int ? value-of<T> : string) $value
 */
function conditional($input, $value): void {}

function conditionalCalls(): void
{
    conditional(0, 0);
    conditional([], 'valid');
    conditional([], 0);
    parameterConditional(0, 0);
    parameterConditional([], 'valid');
    parameterConditional(value: 0, input: 0);
}

/**
 * @template T
 * @param T $input
 * @param ($input is int ? value-of<T> : string) $value
 */
function parameterConditional($input, $value): void {}
