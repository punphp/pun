<?php

namespace Illuminate\Database\Eloquent\Concerns;

use Illuminate\Support\Arr;
use Illuminate\Support\Onceable;
use WeakMap;

trait PreventsCircularRecursion
{
    /**
     * The cache of objects processed to prevent infinite recursion.
     *
     * @var WeakMap<static, array<string, mixed>>
     */
    protected static $recursionCache;

    /**
     * Prevent a method from being called multiple times on the same object within the same call stack.
     *
     * @param  callable  $callback
     * @param  mixed  $default
     * @return mixed
     */
    protected function withoutRecursion($callback, $default = null)
    {}

    /**
     * Remove an entry from the recursion cache for an object.
     *
     * @param  object  $object
     * @param  string  $hash
     */
    protected static function clearRecursiveCallValue($object, string $hash)
    {}

    /**
     * Get the stack of methods being called recursively for the current object.
     *
     * @param  object  $object
     * @return array
     */
    protected static function getRecursiveCallStack($object): array
    {}

    /**
     * Get the current recursion cache being used by the model.
     *
     * @return WeakMap
     */
    protected static function getRecursionCache()
    {}

    /**
     * Set a value in the recursion cache for the given object and method.
     *
     * @param  object  $object
     * @param  string  $hash
     * @param  mixed  $value
     * @return mixed
     */
    protected static function setRecursiveCallValue($object, string $hash, $value)
    {}
}
