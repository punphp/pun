<?php

namespace Illuminate\Database\Eloquent\Concerns;

use Illuminate\Database\Eloquent\Attributes\Initialize;
use Illuminate\Database\Eloquent\Attributes\Table;
use Illuminate\Database\Eloquent\Attributes\WithoutTimestamps;
use Illuminate\Support\Arr;
use Illuminate\Support\Facades\Date;

trait HasTimestamps
{
    /**
     * Indicates if the model should be timestamped.
     *
     * @var bool
     */
    public $timestamps = true;

    /**
     * The list of models classes that have timestamps temporarily disabled.
     *
     * @var array
     */
    protected static $ignoreTimestampsOn = [];

    /**
     * Initialize the HasTimestamps trait.
     *
     * @return void
     */
    #[Initialize]
    public function initializeHasTimestamps()
    {}

    /**
     * Update the model's update timestamp.
     *
     * @param  array|string|null  $attribute
     * @return bool
     */
    public function touch($attribute = null)
    {}

    /**
     * Update the model's update timestamp without raising any events.
     *
     * @param  array|string|null  $attribute
     * @return bool
     */
    public function touchQuietly($attribute = null)
    {}

    /**
     * Update the creation and update timestamps.
     *
     * @return $this
     */
    public function updateTimestamps()
    {}

    /**
     * Set the value of the "created at" attribute.
     *
     * @param  mixed  $value
     * @return $this
     */
    public function setCreatedAt($value)
    {}

    /**
     * Set the value of the "updated at" attribute.
     *
     * @param  mixed  $value
     * @return $this
     */
    public function setUpdatedAt($value)
    {}

    /**
     * Get a fresh timestamp for the model.
     *
     * @return \Illuminate\Support\Carbon
     */
    public function freshTimestamp()
    {}

    /**
     * Get a fresh timestamp for the model.
     *
     * @return string
     */
    public function freshTimestampString()
    {}

    /**
     * Determine if the model uses timestamps.
     *
     * @return bool
     */
    public function usesTimestamps()
    {}

    /**
     * Get the name of the "created at" column.
     *
     * @return string|null
     */
    public function getCreatedAtColumn()
    {}

    /**
     * Get the name of the "updated at" column.
     *
     * @return string|null
     */
    public function getUpdatedAtColumn()
    {}

    /**
     * Get the fully-qualified "created at" column.
     *
     * @return string|null
     */
    public function getQualifiedCreatedAtColumn()
    {}

    /**
     * Get the fully-qualified "updated at" column.
     *
     * @return string|null
     */
    public function getQualifiedUpdatedAtColumn()
    {}

    /**
     * Disable timestamps for the current class during the given callback scope.
     *
     * @template TReturn
     *
     * @param  (callable(): TReturn)  $callback
     * @return TReturn
     */
    public static function withoutTimestamps(callable $callback)
    {}

    /**
     * Disable timestamps for the given model classes during the given callback scope.
     *
     * @template TReturn
     *
     * @param  array  $models
     * @param  callable(): TReturn  $callback
     * @return TReturn
     */
    public static function withoutTimestampsOn($models, $callback)
    {}

    /**
     * Determine if the given model is ignoring timestamps / touches.
     *
     * @param  string|null  $class
     * @return bool
     */
    public static function isIgnoringTimestamps($class = null)
    {}
}
