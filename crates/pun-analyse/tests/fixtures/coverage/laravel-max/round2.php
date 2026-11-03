<?php

namespace LaravelMax\Round2;

/**
 * @template TWhenParameter
 * @template TWhenReturnType
 * @param  TWhenParameter|null  $value
 * @param  callable(TWhenParameter): TWhenReturnType  $callback
 * @return TWhenReturnType|null
 */
function when($value, callable $callback)
{
    $value = $value instanceof \Closure ? $value() : $value;
    if ($value) {
        return null;
    }

    return $callback($value);
}

/**
 * @template T
 */
class Box
{
    /** @return T */
    public function get(int $i)
    {
        return $i;
    }
}

function reflected(\ReflectionMethod $method): string
{
    $type = $method->getReturnType();
    if ($type === null) {
        return '';
    }

    return $type->getName();
}

function emptyOffset(mixed $m): string
{
    $w = empty($m['a']) ? '' : 'x';

    return $w . $m['b'];
}

function issetOffset(mixed $m): mixed
{
    if (isset($m['a'])) {
        return $m['b'];
    }

    return null;
}

/** @param class-string $class */
function relations(object $object, string $class, string $name): bool
{
    return in_array('A', class_implements($object))
        && in_array('B', class_uses($class))
        && in_array('C', class_parents($name));
}

/**
 * @implements \IteratorAggregate<int, int>
 */
class Items implements \IteratorAggregate
{
    public function getIterator(): \ArrayIterator
    {
        return new \ArrayIterator([1]);
    }

    /** @return static */
    public function fresh(): static
    {
        return clone $this;
    }

    public function sum(): int
    {
        $sum = 0;
        foreach ($this->fresh() as $value) {
            $sum++;
        }

        return $sum;
    }
}

function issetRead(mixed $claims): ?int
{
    return isset($claims['exp']) ? (int) $claims['exp'] : null;
}

/**
 * @template TValue
 * @template TArgs
 * @param  TValue|\Closure(TArgs): TValue  $value
 * @param  TArgs  ...$args
 * @return TValue
 */
function value($value, ...$args)
{
    return $value instanceof \Closure ? $value(...$args) : $value;
}

/**
 * @template TDefault
 * @param  TDefault|(\Closure(): TDefault)  $default
 * @return TDefault|int
 */
function firstOr(int $i, $default = null)
{
    return $i > 0 ? $i : value($default);
}

/** @template T */
class Query
{
}

class Model
{
    /** @return Query<static> */
    public function newQuery(): Query
    {
        return new Query();
    }
}

/**
 * @template TModel of Model
 * @param TModel $model
 * @return Query<TModel>
 */
function restoration($model): Query
{
    return $model->newQuery();
}

/**
 * @param iterable<array-key, string> $items
 */
function strings(iterable $items): void
{
}

/** @param Items $items */
function traversed(Items $items, Mixed_ $mixed): void
{
    strings($items);
    strings($mixed);
}

/**
 * @implements \IteratorAggregate<array-key, mixed>
 */
class Mixed_ implements \IteratorAggregate
{
    public function getIterator(): \ArrayIterator
    {
        return new \ArrayIterator([]);
    }
}

/**
 * @template T
 * @param class-string<T> $class
 * @return T
 */
function make(string $class)
{
    return new $class();
}

/**
 * @template T of object
 * @param T $object
 * @return class-string<T>
 */
function classOf($object)
{
    return get_class($object);
}

class Base
{
    /** @var mixed */
    protected $fields = [];

    public function populate(): void
    {
        $this->fields = ['foo' => 'bar'];
    }
}

class Child extends Base
{
    public function populate(): void
    {
        if (empty($this->fields)) {
            parent::populate();
            unset($this->fields['foo']);
        }
    }
}

/**
 * @template T of \BackedEnum
 * @param class-string<T> $class
 * @param value-of<T> $value
 */
function fromValue(string $class, mixed $value): void
{
}

/**
 * @template Q of \BackedEnum
 * @param class-string<Q> $class
 * @param value-of<Q> $value
 */
function forward(string $class, mixed $value): void
{
    fromValue($class, $value);
}
