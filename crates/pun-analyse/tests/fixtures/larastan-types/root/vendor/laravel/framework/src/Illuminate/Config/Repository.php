<?php

namespace Illuminate\Config;

use ArrayAccess;
use Illuminate\Contracts\Config\Repository as ConfigContract;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;

class Repository implements ArrayAccess, ConfigContract
{
    use Macroable;

    /**
     * All of the configuration items.
     *
     * @var array<string,mixed>
     */
    protected $items = [];

    /**
     * Create a new configuration repository.
     *
     * @param  array  $items
     */
    public function __construct(array $items = [])
    {}

    /**
     * Determine if the given configuration value exists.
     *
     * @param  string  $key
     * @return bool
     */
    public function has($key)
    {}

    /**
     * Get the specified configuration value.
     *
     * @param  array|string  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function get($key, $default = null)
    {}

    /**
     * Get many configuration values.
     *
     * @param  array<string|int,mixed>  $keys
     * @return array<string,mixed>
     */
    public function getMany($keys)
    {}

    /**
     * Get the specified string configuration value.
     *
     * @param  string  $key
     * @param  (\Closure():(string|null))|string|null  $default
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    public function string(string $key, $default = null): string
    {}

    /**
     * Get the specified integer configuration value.
     *
     * @param  string  $key
     * @param  (\Closure():(int|null))|int|null  $default
     * @return int
     *
     * @throws \InvalidArgumentException
     */
    public function integer(string $key, $default = null): int
    {}

    /**
     * Get the specified float configuration value.
     *
     * @param  string  $key
     * @param  (\Closure():(float|null))|float|null  $default
     * @return float
     *
     * @throws \InvalidArgumentException
     */
    public function float(string $key, $default = null): float
    {}

    /**
     * Get the specified boolean configuration value.
     *
     * @param  string  $key
     * @param  (\Closure():(bool|null))|bool|null  $default
     * @return bool
     *
     * @throws \InvalidArgumentException
     */
    public function boolean(string $key, $default = null): bool
    {}

    /**
     * Get the specified array configuration value.
     *
     * @param  string  $key
     * @param  (\Closure():(array<array-key, mixed>|null))|array<array-key, mixed>|null  $default
     * @return array<array-key, mixed>
     *
     * @throws \InvalidArgumentException
     */
    public function array(string $key, $default = null): array
    {}

    /**
     * Get the specified array configuration value as a collection.
     *
     * @param  string  $key
     * @param  (\Closure():(array<array-key, mixed>|null))|array<array-key, mixed>|null  $default
     * @return Collection<array-key, mixed>
     */
    public function collection(string $key, $default = null): Collection
    {}

    /**
     * Set a given configuration value.
     *
     * @param  array|string  $key
     * @param  mixed  $value
     * @return void
     */
    public function set($key, $value = null)
    {}

    /**
     * Prepend a value onto an array configuration value.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return void
     */
    public function prepend($key, $value)
    {}

    /**
     * Push a value onto an array configuration value.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return void
     */
    public function push($key, $value)
    {}

    /**
     * Get all of the configuration items for the application.
     *
     * @return array<string, mixed>
     */
    public function all()
    {}

    /**
     * Determine if the given configuration option exists.
     *
     * @param  string  $offset
     * @return bool
     */
    public function offsetExists($offset): bool
    {}

    /**
     * Get a configuration option.
     *
     * @param  string  $offset
     * @return mixed
     */
    public function offsetGet($offset): mixed
    {}

    /**
     * Set a configuration option.
     *
     * @param  string  $offset
     * @param  mixed  $value
     * @return void
     */
    public function offsetSet($offset, $value): void
    {}

    /**
     * Unset a configuration option.
     *
     * @param  string  $offset
     * @return void
     */
    public function offsetUnset($offset): void
    {}
}
