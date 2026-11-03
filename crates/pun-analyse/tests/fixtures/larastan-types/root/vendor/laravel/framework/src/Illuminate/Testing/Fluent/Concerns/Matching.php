<?php

namespace Illuminate\Testing\Fluent\Concerns;

use Closure;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Support\Collection;
use PHPUnit\Framework\Assert as PHPUnit;

use function Illuminate\Support\enum_value;

trait Matching
{
    /**
     * Asserts that the property matches the expected value.
     *
     * @param  string  $key
     * @param  mixed|\Closure  $expected
     * @return $this
     */
    public function where(string $key, $expected): static
    {}

    /**
     * Asserts that the property does not match the expected value.
     *
     * @param  string  $key
     * @param  mixed|\Closure  $expected
     * @return $this
     */
    public function whereNot(string $key, $expected): static
    {}

    /**
     * Asserts that the property is null.
     *
     * @param  string  $key
     * @return $this
     */
    public function whereNull(string $key): static
    {}

    /**
     * Asserts that the property is not null.
     *
     * @param  string  $key
     * @return $this
     */
    public function whereNotNull(string $key): static
    {}

    /**
     * Asserts that all properties match their expected values.
     *
     * @param  array  $bindings
     * @return $this
     */
    public function whereAll(array $bindings): static
    {}

    /**
     * Asserts that the property is of the expected type.
     *
     * @param  string  $key
     * @param  string|array  $expected
     * @return $this
     */
    public function whereType(string $key, $expected): static
    {}

    /**
     * Asserts that all properties are of their expected types.
     *
     * @param  array  $bindings
     * @return $this
     */
    public function whereAllType(array $bindings): static
    {}

    /**
     * Asserts that the property contains the expected values.
     *
     * @param  string  $key
     * @param  mixed  $expected
     * @return $this
     */
    public function whereContains(string $key, $expected)
    {}

    /**
     * Ensures that all properties are sorted the same way, recursively.
     *
     * @param  mixed  $value
     * @return void
     */
    protected function ensureSorted(&$value): void
    {}

    /**
     * Compose the absolute "dot" path to the given key.
     *
     * @param  string  $key
     * @return string
     */
    abstract protected function dotPath(string $key = ''): string;

    /**
     * Ensure that the given prop exists.
     *
     * @param  string  $key
     * @param  null  $value
     * @param  \Closure|null  $scope
     * @return $this
     */
    abstract public function has(string $key, $value = null, ?Closure $scope = null);

    /**
     * Retrieve a prop within the current scope using "dot" notation.
     *
     * @param  string|null  $key
     * @return mixed
     */
    abstract protected function prop(?string $key = null);
}
