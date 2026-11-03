<?php

namespace Illuminate\Support;

use ArrayIterator;
use Illuminate\Contracts\Support\ValidatedData;
use Illuminate\Support\Traits\Dumpable;
use Illuminate\Support\Traits\InteractsWithData;
use Traversable;

class ValidatedInput implements ValidatedData
{
    use Dumpable, InteractsWithData;

    /**
     * The underlying input.
     *
     * @var array
     */
    protected $input;

    /**
     * Create a new validated input container.
     *
     * @param  array  $input
     */
    public function __construct(array $input)
    {}

    /**
     * Merge the validated input with the given array of additional data.
     *
     * @param  array  $items
     * @return static
     */
    public function merge(array $items)
    {}

    /**
     * Get the raw, underlying input array.
     *
     * @param  mixed  $keys
     * @return array
     */
    public function all($keys = null)
    {}

    /**
     * Retrieve data from the instance.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return mixed
     */
    protected function data($key = null, $default = null)
    {}

    /**
     * Get the keys for all of the input.
     *
     * @return array
     */
    public function keys()
    {}

    /**
     * Retrieve an input item from the validated inputs.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function input($key = null, $default = null)
    {}

    /**
     * Retrieve a file from the validated inputs.
     *
     * @param  string  $key
     * @param  mixed  $default
     * @return \Illuminate\Http\UploadedFile|null
     */
    public function file($key, $default = null)
    {}

    /**
     * Dump the items.
     *
     * @param  mixed  ...$keys
     * @return $this
     */
    public function dump(...$keys)
    {}

    /**
     * Get the instance as an array.
     *
     * @return array
     */
    public function toArray()
    {}

    /**
     * Dynamically access input data.
     *
     * @param  string  $name
     * @return mixed
     */
    public function __get($name)
    {}

    /**
     * Dynamically set input data.
     *
     * @param  string  $name
     * @param  mixed  $value
     * @return mixed
     */
    public function __set($name, $value)
    {}

    /**
     * Determine if an input item is set.
     *
     * @param  string  $name
     * @return bool
     */
    public function __isset($name)
    {}

    /**
     * Remove an input item.
     *
     * @param  string  $name
     * @return void
     */
    public function __unset($name)
    {}

    /**
     * Determine if an item exists at an offset.
     *
     * @param  mixed  $key
     * @return bool
     */
    public function offsetExists($key): bool
    {}

    /**
     * Get an item at a given offset.
     *
     * @param  mixed  $key
     * @return mixed
     */
    public function offsetGet($key): mixed
    {}

    /**
     * Set the item at a given offset.
     *
     * @param  mixed  $key
     * @param  mixed  $value
     * @return void
     */
    public function offsetSet($key, $value): void
    {}

    /**
     * Unset the item at a given offset.
     *
     * @param  string  $key
     * @return void
     */
    public function offsetUnset($key): void
    {}

    /**
     * Get an iterator for the input.
     *
     * @return \ArrayIterator
     */
    public function getIterator(): Traversable
    {}
}
