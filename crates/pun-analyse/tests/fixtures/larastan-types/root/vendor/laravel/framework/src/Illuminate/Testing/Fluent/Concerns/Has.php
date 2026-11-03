<?php

namespace Illuminate\Testing\Fluent\Concerns;

use Closure;
use Illuminate\Support\Arr;
use PHPUnit\Framework\Assert as PHPUnit;

trait Has
{
    /**
     * Assert that the prop is of the expected size.
     *
     * @param  string|int  $key
     * @param  int|null  $length
     * @return $this
     */
    public function count($key, ?int $length = null): static
    {}

    /**
     * Assert that the prop size is between a given minimum and maximum.
     *
     * @param  int|string  $min
     * @param  int|string  $max
     * @return $this
     */
    public function countBetween(int|string $min, int|string $max): static
    {}

    /**
     * Ensure that the given prop exists.
     *
     * @param  string|int  $key
     * @param  int|\Closure|null  $length
     * @param  \Closure|null  $callback
     * @return $this
     */
    public function has($key, $length = null, ?Closure $callback = null): static
    {}

    /**
     * Assert that all of the given props exist.
     *
     * @param  array|string  $key
     * @return $this
     */
    public function hasAll($key): static
    {}

    /**
     * Assert that at least one of the given props exists.
     *
     * @param  array|string  $key
     * @return $this
     */
    public function hasAny($key): static
    {}

    /**
     * Assert that none of the given props exist.
     *
     * @param  array|string  $key
     * @return $this
     */
    public function missingAll($key): static
    {}

    /**
     * Assert that the given prop does not exist.
     *
     * @param  string  $key
     * @return $this
     */
    public function missing(string $key): static
    {}

    /**
     * Compose the absolute "dot" path to the given key.
     *
     * @param  string  $key
     * @return string
     */
    abstract protected function dotPath(string $key = ''): string;

    /**
     * Marks the property as interacted.
     *
     * @param  string  $key
     * @return void
     */
    abstract protected function interactsWith(string $key): void;

    /**
     * Retrieve a prop within the current scope using "dot" notation.
     *
     * @param  string|null  $key
     * @return mixed
     */
    abstract protected function prop(?string $key = null);

    /**
     * Instantiate a new "scope" at the path of the given key.
     *
     * @param  string  $key
     * @param  \Closure  $callback
     * @return $this
     */
    abstract protected function scope(string $key, Closure $callback);

    /**
     * Disables the interaction check.
     *
     * @return $this
     */
    abstract public function etc();

    /**
     * Instantiate a new "scope" on the first element.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    abstract public function first(Closure $callback);
}
