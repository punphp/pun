<?php

namespace Illuminate\Support;

use ArrayAccess;
use ArrayIterator;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Contracts\Support\Jsonable;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\InteractsWithData;
use Illuminate\Support\Traits\Macroable;
use IteratorAggregate;
use JsonSerializable;
use Traversable;

/**
 * @template TKey of array-key
 * @template TValue
 *
 * @implements \Illuminate\Contracts\Support\Arrayable<TKey, TValue>
 * @implements \ArrayAccess<TKey, TValue>
 */
class Fluent implements Arrayable, ArrayAccess, IteratorAggregate, Jsonable, JsonSerializable
{
    use Conditionable, InteractsWithData, Macroable {
        __call as macroCall;
    }

    /**
     * All of the attributes set on the fluent instance.
     *
     * @var array<TKey, TValue>
     */
    protected $attributes = [];

    /**
     * Create a new fluent instance.
     *
     * @param  iterable<TKey, TValue>  $attributes
     */
    public function __construct($attributes = [])
    {}

    /**
     * Create a new fluent instance.
     *
     * @param  iterable<TKey, TValue>  $attributes
     * @return static
     */
    public static function make($attributes = [])
    {}

    /**
     * Get an attribute from the fluent instance using "dot" notation.
     *
     * @template TGetDefault
     *
     * @param  TKey  $key
     * @param  TGetDefault|(\Closure(): TGetDefault)  $default
     * @return TValue|TGetDefault
     */
    public function get($key, $default = null)
    {}

    /**
     * Set an attribute on the fluent instance using "dot" notation.
     *
     * @param  TKey  $key
     * @param  TValue  $value
     * @return $this
     */
    public function set($key, $value)
    {}

    /**
     * Fill the fluent instance with an array of attributes.
     *
     * @param  iterable<TKey, TValue>  $attributes
     * @return $this
     */
    public function fill($attributes)
    {}

    /**
     * Get an attribute from the fluent instance.
     *
     * @param  string  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function value($key, $default = null)
    {}

    /**
     * Get the value of the given key as a new Fluent instance.
     *
     * @param  string  $key
     * @param  mixed  $default
     * @return static
     */
    public function scope($key, $default = null)
    {}

    /**
     * Get all of the attributes from the fluent instance.
     *
     * @param  mixed  $keys
     * @return array<TKey, TValue>
     */
    public function all($keys = null)
    { func_get_args(); }

    /**
     * Get data from the fluent instance.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return mixed
     */
    protected function data($key = null, $default = null)
    {}

    /**
     * Get the attributes from the fluent instance.
     *
     * @return array<TKey, TValue>
     */
    public function getAttributes()
    {}

    /**
     * Convert the fluent instance to an array.
     *
     * @return array<TKey, TValue>
     */
    public function toArray()
    {}

    /**
     * Convert the object into something JSON serializable.
     *
     * @return array<TKey, TValue>
     */
    public function jsonSerialize(): array
    {}

    /**
     * Convert the fluent instance to JSON.
     *
     * @param  int  $options
     * @return string
     */
    public function toJson($options = 0)
    {}

    /**
     * Convert the fluent instance to pretty print formatted JSON.
     *
     * @param  int  $options
     * @return string
     */
    public function toPrettyJson(int $options = 0)
    {}

    /**
     * Determine if the fluent instance is empty.
     *
     * @return bool
     */
    public function isEmpty(): bool
    {}

    /**
     * Determine if the fluent instance is not empty.
     *
     * @return bool
     */
    public function isNotEmpty(): bool
    {}

    /**
     * Determine if the given offset exists.
     *
     * @param  TKey  $offset
     * @return bool
     */
    public function offsetExists($offset): bool
    {}

    /**
     * Get the value for a given offset.
     *
     * @param  TKey  $offset
     * @return TValue|null
     */
    public function offsetGet($offset): mixed
    {}

    /**
     * Set the value at the given offset.
     *
     * @param  TKey  $offset
     * @param  TValue  $value
     * @return void
     */
    public function offsetSet($offset, $value): void
    {}

    /**
     * Unset the value at the given offset.
     *
     * @param  TKey  $offset
     * @return void
     */
    public function offsetUnset($offset): void
    {}

    /**
     * Get an iterator for the attributes.
     *
     * @return ArrayIterator<TKey, TValue>
     */
    public function getIterator(): Traversable
    {}

    /**
     * Handle dynamic calls to the fluent instance to set attributes.
     *
     * @param  TKey  $method
     * @param  array{0?: ?TValue}  $parameters
     * @return $this
     */
    public function __call($method, $parameters)
    {}

    /**
     * Dynamically retrieve the value of an attribute.
     *
     * @param  TKey  $key
     * @return TValue|null
     */
    public function __get($key)
    {}

    /**
     * Dynamically set the value of an attribute.
     *
     * @param  TKey  $key
     * @param  TValue  $value
     * @return void
     */
    public function __set($key, $value)
    {}

    /**
     * Dynamically check if an attribute is set.
     *
     * @param  TKey  $key
     * @return bool
     */
    public function __isset($key)
    {}

    /**
     * Dynamically unset an attribute.
     *
     * @param  TKey  $key
     * @return void
     */
    public function __unset($key)
    {}
}
