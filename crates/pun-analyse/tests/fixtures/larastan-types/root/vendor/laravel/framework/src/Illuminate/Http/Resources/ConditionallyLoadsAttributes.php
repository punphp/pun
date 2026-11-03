<?php

namespace Illuminate\Http\Resources;

use Illuminate\Http\Resources\Attributes\PreserveKeys;
use Illuminate\Support\Arr;
use Illuminate\Support\Stringable;
use ReflectionClass;

trait ConditionallyLoadsAttributes
{
    /**
     * The cached preserve keys attribute values.
     *
     * @var array<class-string, bool>
     */
    protected static $cachedPreserveKeysAttributes = [];

    /**
     * Filter the given data, removing any optional values.
     *
     * @param  array  $data
     * @return array
     */
    protected function filter($data)
    {}

    /**
     * Merge the given data in at the given index.
     *
     * @param  array  $data
     * @param  int  $index
     * @param  array  $merge
     * @param  bool  $numericKeys
     * @return array
     */
    protected function mergeData($data, $index, $merge, $numericKeys)
    {}

    /**
     * Remove the missing values from the filtered data.
     *
     * @param  array  $data
     * @return array
     */
    protected function removeMissingValues($data)
    {}

    /**
     * Retrieve a value if the given "condition" is truthy.
     *
     * @param  bool  $condition
     * @param  mixed  $value
     * @param  mixed  $default
     * @return \Illuminate\Http\Resources\MissingValue|mixed
     */
    protected function when($condition, $value, $default = new MissingValue)
    {}

    /**
     * Retrieve a value if the given "condition" is falsy.
     *
     * @param  bool  $condition
     * @param  mixed  $value
     * @param  mixed  $default
     * @return \Illuminate\Http\Resources\MissingValue|mixed
     */
    public function unless($condition, $value, $default = new MissingValue)
    {}

    /**
     * Merge a value into the array.
     *
     * @param  mixed  $value
     * @return \Illuminate\Http\Resources\MergeValue|mixed
     */
    protected function merge($value)
    {}

    /**
     * Merge a value if the given condition is truthy.
     *
     * @param  bool  $condition
     * @param  mixed  $value
     * @param  mixed  $default
     * @return \Illuminate\Http\Resources\MergeValue|mixed
     */
    protected function mergeWhen($condition, $value, $default = new MissingValue)
    {}

    /**
     * Merge a value unless the given condition is truthy.
     *
     * @param  bool  $condition
     * @param  mixed  $value
     * @param  mixed  $default
     * @return \Illuminate\Http\Resources\MergeValue|mixed
     */
    protected function mergeUnless($condition, $value, $default = new MissingValue)
    {}

    /**
     * Merge the given attributes.
     *
     * @param  array  $attributes
     * @return \Illuminate\Http\Resources\MergeValue
     */
    protected function attributes($attributes)
    {}

    /**
     * Retrieve an attribute if it exists on the resource.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $default
     * @return \Illuminate\Http\Resources\MissingValue|mixed
     */
    public function whenHas($attribute, $value = null, $default = new MissingValue)
    {}

    /**
     * Retrieve a model attribute if it is null.
     *
     * @param  mixed  $value
     * @param  mixed  $default
     * @return \Illuminate\Http\Resources\MissingValue|mixed
     */
    protected function whenNull($value, $default = new MissingValue)
    {}

    /**
     * Retrieve a model attribute if it is not null.
     *
     * @param  mixed  $value
     * @param  mixed  $default
     * @return \Illuminate\Http\Resources\MissingValue|mixed
     */
    protected function whenNotNull($value, $default = new MissingValue)
    {}

    /**
     * Retrieve an accessor when it has been appended.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @param  mixed  $default
     * @return \Illuminate\Http\Resources\MissingValue|mixed
     */
    protected function whenAppended($attribute, $value = null, $default = new MissingValue)
    {}

    /**
     * Retrieve a relationship if it has been loaded.
     *
     * @param  string  $relationship
     * @param  mixed  $value
     * @param  mixed  $default
     * @return \Illuminate\Http\Resources\MissingValue|mixed
     */
    protected function whenLoaded($relationship, $value = null, $default = new MissingValue)
    {}

    /**
     * Retrieve a relationship count if it exists.
     *
     * @param  string  $relationship
     * @param  mixed  $value
     * @param  mixed  $default
     * @return \Illuminate\Http\Resources\MissingValue|mixed
     */
    public function whenCounted($relationship, $value = null, $default = new MissingValue)
    {}

    /**
     * Retrieve a relationship aggregated value if it exists.
     *
     * @param  string  $relationship
     * @param  string  $column
     * @param  string  $aggregate
     * @param  mixed  $value
     * @param  mixed  $default
     * @return \Illuminate\Http\Resources\MissingValue|mixed
     */
    public function whenAggregated($relationship, $column, $aggregate, $value = null, $default = new MissingValue)
    {}

    /**
     * Retrieve a relationship existence check if it exists.
     *
     * @param  string  $relationship
     * @param  mixed  $value
     * @param  mixed  $default
     * @return \Illuminate\Http\Resources\MissingValue|mixed
     */
    public function whenExistsLoaded($relationship, $value = null, $default = new MissingValue)
    {}

    /**
     * Execute a callback if the given pivot table has been loaded.
     *
     * @param  string  $table
     * @param  mixed  $value
     * @param  mixed  $default
     * @return \Illuminate\Http\Resources\MissingValue|mixed
     */
    protected function whenPivotLoaded($table, $value, $default = new MissingValue)
    {}

    /**
     * Execute a callback if the given pivot table with a custom accessor has been loaded.
     *
     * @param  string  $accessor
     * @param  string  $table
     * @param  mixed  $value
     * @param  mixed  $default
     * @return \Illuminate\Http\Resources\MissingValue|mixed
     */
    protected function whenPivotLoadedAs($accessor, $table, $value, $default = new MissingValue)
    {}

    /**
     * Determine if the resource has the specified pivot table loaded.
     *
     * @param  string  $table
     * @return bool
     */
    protected function hasPivotLoaded($table)
    {}

    /**
     * Determine if the resource has the specified pivot table loaded with a custom accessor.
     *
     * @param  string  $accessor
     * @param  string  $table
     * @return bool
     */
    protected function hasPivotLoadedAs($accessor, $table)
    {}

    /**
     * Transform the given value if it is present.
     *
     * @param  mixed  $value
     * @param  callable  $callback
     * @param  mixed  $default
     * @return mixed
     */
    protected function transform($value, callable $callback, $default = new MissingValue)
    {}
}
