<?php

namespace Illuminate\Support;

use WeakMap;
use WeakReference;

class Once
{
    /**
     * The current globally used instance.
     *
     * @var static|null
     */
    protected static ?self $instance = null;

    /**
     * Indicates if the once instance is enabled.
     *
     * @var bool
     */
    protected static bool $enabled = true;

    /**
     * Create a new once instance.
     *
     * @param  \WeakMap<object, array<string, array{value: mixed, objects: array<int, \WeakReference<object>>}>>  $values
     */
    protected function __construct(protected WeakMap $values)
    {}

    /**
     * Create a new once instance.
     *
     * @return static
     */
    public static function instance()
    {}

    /**
     * Get the value of the given onceable.
     *
     * @param  Onceable  $onceable
     * @return mixed
     */
    public function value(Onceable $onceable)
    {}

    /**
     * Determine if the cached references still point to the given objects.
     *
     * Object hashes are based on object IDs, which PHP reuses once an object is destroyed.
     *
     * @param  array<int, \WeakReference<object>>  $references
     * @param  array<int, object>  $objects
     * @return bool
     */
    protected function usesSameObjects(array $references, array $objects)
    {}

    /**
     * Re-enable the once instance if it was disabled.
     *
     * @return void
     */
    public static function enable()
    {}

    /**
     * Disable the once instance.
     *
     * @return void
     */
    public static function disable()
    {}

    /**
     * Flush the once instance.
     *
     * @return void
     */
    public static function flush()
    {}
}
