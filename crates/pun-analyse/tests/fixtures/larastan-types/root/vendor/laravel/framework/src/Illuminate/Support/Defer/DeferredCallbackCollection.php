<?php

namespace Illuminate\Support\Defer;

use ArrayAccess;
use Closure;
use Countable;
use Illuminate\Support\Collection;

class DeferredCallbackCollection implements ArrayAccess, Countable
{
    /**
     * All of the deferred callbacks.
     *
     * @var array
     */
    protected array $callbacks = [];

    /**
     * Get the first callback in the collection.
     *
     * @return callable
     */
    public function first()
    {}

    /**
     * Invoke the deferred callbacks.
     *
     * @return void
     */
    public function invoke(): void
    {}

    /**
     * Invoke the deferred callbacks if the given truth test evaluates to true.
     *
     * @param  \Closure|null  $when
     * @return void
     */
    public function invokeWhen(?Closure $when = null): void
    {}

    /**
     * Remove any deferred callbacks with the given name.
     *
     * @param  string  $name
     * @return void
     */
    public function forget(string $name): void
    {}

    /**
     * Remove any duplicate callbacks.
     *
     * @return $this
     */
    protected function forgetDuplicates(): static
    {}

    /**
     * Determine if the collection has a callback with the given key.
     *
     * @param  mixed  $offset
     * @return bool
     */
    public function offsetExists(mixed $offset): bool
    {}

    /**
     * Get the callback with the given key.
     *
     * @param  mixed  $offset
     * @return mixed
     */
    public function offsetGet(mixed $offset): mixed
    {}

    /**
     * Set the callback with the given key.
     *
     * @param  mixed  $offset
     * @param  mixed  $value
     * @return void
     */
    public function offsetSet(mixed $offset, mixed $value): void
    {}

    /**
     * Remove the callback with the given key from the collection.
     *
     * @param  mixed  $offset
     * @return void
     */
    public function offsetUnset(mixed $offset): void
    {}

    /**
     * Determine how many callbacks are in the collection.
     *
     * @return int
     */
    public function count(): int
    {}
}
