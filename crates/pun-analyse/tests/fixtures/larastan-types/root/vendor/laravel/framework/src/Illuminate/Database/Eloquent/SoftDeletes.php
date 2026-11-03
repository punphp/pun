<?php

namespace Illuminate\Database\Eloquent;

use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Support\Collection as BaseCollection;

/**
 * @method static \Illuminate\Database\Eloquent\Builder<static> withTrashed(bool $withTrashed = true)
 * @method static \Illuminate\Database\Eloquent\Builder<static> onlyTrashed()
 * @method static \Illuminate\Database\Eloquent\Builder<static> withoutTrashed()
 * @method static static restoreOrCreate(array<string, mixed> $attributes = [], array<string, mixed> $values = [])
 * @method static static createOrRestore(array<string, mixed> $attributes = [], array<string, mixed> $values = [])
 */
trait SoftDeletes
{
    /**
     * Indicates if the model is currently force deleting.
     *
     * @var bool
     */
    protected $forceDeleting = false;

    /**
     * Boot the soft deleting trait for a model.
     *
     * @return void
     */
    public static function bootSoftDeletes()
    {}

    /**
     * Initialize the soft deleting trait for an instance.
     *
     * @return void
     */
    public function initializeSoftDeletes()
    {}

    /**
     * Force a hard delete on a soft deleted model.
     *
     * @return bool|null
     */
    public function forceDelete()
    {}

    /**
     * Force a hard delete on a soft deleted model without raising any events.
     *
     * @return bool|null
     */
    public function forceDeleteQuietly()
    {}

    /**
     * Destroy the models for the given IDs.
     *
     * @param  \Illuminate\Support\Collection|array|int|string  $ids
     * @return int
     */
    public static function forceDestroy($ids)
    {}

    /**
     * Perform the actual delete query on this model instance.
     *
     * @return mixed
     */
    protected function performDeleteOnModel()
    {}

    /**
     * Perform the actual delete query on this model instance.
     *
     * @return void
     */
    protected function runSoftDelete()
    {}

    /**
     * Restore a soft-deleted model instance.
     *
     * @return bool
     */
    public function restore()
    {}

    /**
     * Restore a soft-deleted model instance without raising any events.
     *
     * @return bool
     */
    public function restoreQuietly()
    {}

    /**
     * Determine if the model instance has been soft-deleted.
     *
     * @return bool
     */
    public function trashed()
    {}

    /**
     * Register a "softDeleted" model event callback with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|class-string  $callback
     * @return void
     */
    public static function softDeleted($callback)
    {}

    /**
     * Register a "restoring" model event callback with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|class-string  $callback
     * @return void
     */
    public static function restoring($callback)
    {}

    /**
     * Register a "restored" model event callback with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|class-string  $callback
     * @return void
     */
    public static function restored($callback)
    {}

    /**
     * Register a "forceDeleting" model event callback with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|class-string  $callback
     * @return void
     */
    public static function forceDeleting($callback)
    {}

    /**
     * Register a "forceDeleted" model event callback with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|class-string  $callback
     * @return void
     */
    public static function forceDeleted($callback)
    {}

    /**
     * Determine if the model is currently force deleting.
     *
     * @return bool
     */
    public function isForceDeleting()
    {}

    /**
     * Get the name of the "deleted at" column.
     *
     * @return string
     */
    public function getDeletedAtColumn()
    {}

    /**
     * Get the fully-qualified "deleted at" column.
     *
     * @return string
     */
    public function getQualifiedDeletedAtColumn()
    {}
}
