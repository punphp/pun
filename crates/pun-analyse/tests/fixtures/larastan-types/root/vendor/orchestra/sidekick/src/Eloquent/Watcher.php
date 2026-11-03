<?php

namespace Orchestra\Sidekick\Eloquent;

use Illuminate\Database\Eloquent\Model;
use WeakMap;

/**
 * @internal
 */
class Watcher
{
    /**
     * The cache instance.
     *
     * @var \WeakMap<\Illuminate\Database\Eloquent\Model, array<string, mixed>>|null
     */
    protected static ?WeakMap $cache = null;

    /**
     * Get the watcher store.
     *
     * @return \WeakMap<\Illuminate\Database\Eloquent\Model, array<string, mixed>>
     */
    public static function store(): WeakMap
    {}

    /**
     * Submit a model snapshot.
     *
     * @return array<string, mixed>
     */
    public static function snapshot(Model $model): ?array
    {}

    /**
     * Flush the instance states.
     */
    public static function flushState(): void
    {}
}
