<?php

namespace Illuminate\Support;

use Closure;
use Illuminate\Contracts\Support\HasOnceHash;
use Laravel\SerializableClosure\Support\ReflectionClosure;

class Onceable
{
    /**
     * Create a new onceable instance.
     *
     * @param  string  $hash
     * @param  object|null  $object
     * @param  callable  $callable
     */
    public function __construct(
        public string $hash,
        public ?object $object,
        public $callable,
    ) {}

    /**
     * Tries to create a new onceable instance from the given trace.
     *
     * @param  array<int, array<string, mixed>>  $trace
     * @return static|null
     */
    public static function tryFromTrace(array $trace, callable $callable)
    {}

    /**
     * Get the objects used by the callable, which are hashed by their identity.
     *
     * @return array<int, object>
     */
    public function objects()
    {}

    /**
     * Computes the object of the onceable from the given trace, if any.
     *
     * @param  array<int, array<string, mixed>>  $trace
     * @return object|null
     */
    protected static function objectFromTrace(array $trace)
    {}

    /**
     * Computes the hash of the onceable from the given trace.
     *
     * @param  array<int, array<string, mixed>>  $trace
     * @return string|null
     */
    protected static function hashFromTrace(array $trace, callable $callable)
    {}
}
