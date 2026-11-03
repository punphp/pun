<?php

namespace Illuminate\Database\Eloquent\Relations\Concerns;

use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\Pivot;
use Illuminate\Support\Collection as BaseCollection;

use function Illuminate\Support\enum_value;

trait InteractsWithPivotTable
{
    /**
     * Toggles a model (or models) from the parent.
     *
     * Each existing model is detached, and non existing ones are attached.
     *
     * @param  mixed  $ids
     * @param  bool  $touch
     * @return array
     */
    public function toggle($ids, $touch = true)
    {}

    /**
     * Toggles a model (or models) from the parent within a transaction.
     *
     * @param  mixed  $ids
     * @param  bool  $touch
     * @return array
     *
     * @throws \Throwable
     */
    public function toggleOrFail($ids, $touch = true)
    {}

    /**
     * Sync the intermediate tables with a list of IDs without detaching.
     *
     * @param  \Illuminate\Support\Collection|\Illuminate\Database\Eloquent\Model|array|int|string  $ids
     * @return array{attached: array, detached: array, updated: array}
     */
    public function syncWithoutDetaching($ids)
    {}

    /**
     * Sync the intermediate tables with a list of IDs or collection of models.
     *
     * @param  \Illuminate\Support\Collection|\Illuminate\Database\Eloquent\Model|array|int|string  $ids
     * @param  bool  $detaching
     * @return array{attached: array, detached: array, updated: array}
     */
    public function sync($ids, $detaching = true)
    {}

    /**
     * Sync the intermediate tables with a list of IDs or collection of models within a transaction.
     *
     * @param  \Illuminate\Support\Collection|\Illuminate\Database\Eloquent\Model|array  $ids
     * @param  bool  $detaching
     * @return array{attached: array, detached: array, updated: array}
     *
     * @throws \Throwable
     */
    public function syncOrFail($ids, $detaching = true)
    {}

    /**
     * Sync the intermediate tables with a list of IDs without detaching within a transaction.
     *
     * @param  \Illuminate\Support\Collection|\Illuminate\Database\Eloquent\Model|array  $ids
     * @return array{attached: array, detached: array, updated: array}
     *
     * @throws \Throwable
     */
    public function syncWithoutDetachingOrFail($ids)
    {}

    /**
     * Sync the intermediate tables with a list of IDs or collection of models with the given pivot values.
     *
     * @param  \Illuminate\Support\Collection|\Illuminate\Database\Eloquent\Model|array|int|string  $ids
     * @param  array  $values
     * @param  bool  $detaching
     * @return array{attached: array, detached: array, updated: array}
     */
    public function syncWithPivotValues($ids, array $values, bool $detaching = true)
    {}

    /**
     * Sync the intermediate tables with a list of IDs with the given pivot values within a transaction.
     *
     * @param  \Illuminate\Support\Collection|\Illuminate\Database\Eloquent\Model|array|int|string  $ids
     * @param  array  $values
     * @param  bool  $detaching
     * @return array{attached: array, detached: array, updated: array}
     *
     * @throws \Throwable
     */
    public function syncWithPivotValuesOrFail($ids, array $values, bool $detaching = true)
    {}

    /**
     * Format the sync / toggle record list so that it is keyed by ID.
     *
     * @param  array  $records
     * @return array
     */
    protected function formatRecordsList(array $records)
    {}

    /**
     * Attach all of the records that aren't in the given current records.
     *
     * @param  array  $records
     * @param  array  $current
     * @param  bool  $touch
     * @return array
     */
    protected function attachNew(array $records, array $current, $touch = true)
    {}

    /**
     * Update an existing pivot record on the table.
     *
     * @param  mixed  $id
     * @param  array  $attributes
     * @param  bool  $touch
     * @return int
     */
    public function updateExistingPivot($id, array $attributes, $touch = true)
    {}

    /**
     * Update an existing pivot record on the table within a transaction.
     *
     * @param  mixed  $id
     * @param  array  $attributes
     * @param  bool  $touch
     * @return int
     *
     * @throws \Throwable
     */
    public function updateExistingPivotOrFail($id, array $attributes, $touch = true)
    {}

    /**
     * Update an existing pivot record on the table via a custom class.
     *
     * @param  mixed  $id
     * @param  array  $attributes
     * @param  bool  $touch
     * @return int
     */
    protected function updateExistingPivotUsingCustomClass($id, array $attributes, $touch)
    {}

    /**
     * Attach a model to the parent.
     *
     * @param  mixed  $ids
     * @param  array  $attributes
     * @param  bool  $touch
     * @return void
     */
    public function attach($ids, array $attributes = [], $touch = true)
    {}

    /**
     * Attach a model to the parent within a transaction.
     *
     * @param  mixed  $ids
     * @param  array  $attributes
     * @param  bool  $touch
     * @return void
     *
     * @throws \Throwable
     */
    public function attachOrFail($ids, array $attributes = [], $touch = true)
    {}

    /**
     * Attach a model to the parent using a custom class.
     *
     * @param  mixed  $ids
     * @param  array  $attributes
     * @return void
     */
    protected function attachUsingCustomClass($ids, array $attributes)
    {}

    /**
     * Create an array of records to insert into the pivot table.
     *
     * @param  array  $ids
     * @param  array  $attributes
     * @return array
     */
    protected function formatAttachRecords($ids, array $attributes)
    {}

    /**
     * Create a full attachment record payload.
     *
     * @param  int  $key
     * @param  mixed  $value
     * @param  array  $attributes
     * @param  bool  $hasTimestamps
     * @return array
     */
    protected function formatAttachRecord($key, $value, $attributes, $hasTimestamps)
    {}

    /**
     * Get the attach record ID and extra attributes.
     *
     * @param  mixed  $key
     * @param  mixed  $value
     * @param  array  $attributes
     * @return array
     */
    protected function extractAttachIdAndAttributes($key, $value, array $attributes)
    {}

    /**
     * Create a new pivot attachment record.
     *
     * @param  int  $id
     * @param  bool  $timed
     * @return array
     */
    protected function baseAttachRecord($id, $timed)
    {}

    /**
     * Set the creation and update timestamps on an attach record.
     *
     * @param  array  $record
     * @param  bool  $exists
     * @return array
     */
    protected function addTimestampsToAttachment(array $record, $exists = false)
    {}

    /**
     * Determine whether the given column is defined as a pivot column.
     *
     * @param  string  $column
     * @return bool
     */
    public function hasPivotColumn($column)
    {}

    /**
     * Detach models from the relationship.
     *
     * @param  mixed  $ids
     * @param  bool  $touch
     * @return int
     */
    public function detach($ids = null, $touch = true)
    {}

    /**
     * Detach models from the relationship within a transaction.
     *
     * @param  mixed  $ids
     * @param  bool  $touch
     * @return int
     *
     * @throws \Throwable
     */
    public function detachOrFail($ids = null, $touch = true)
    {}

    /**
     * Detach models from the relationship using a custom class.
     *
     * @param  mixed  $ids
     * @return int
     */
    protected function detachUsingCustomClass($ids)
    {}

    /**
     * Get the pivot models that are currently attached.
     *
     * @return \Illuminate\Support\Collection
     */
    protected function getCurrentlyAttachedPivots()
    {}

    /**
     * Get the pivot models that are currently attached, filtered by related model keys.
     *
     * @param  mixed  $ids
     * @return \Illuminate\Support\Collection
     */
    protected function getCurrentlyAttachedPivotsForIds($ids = null)
    {}

    /**
     * Create a new pivot model instance.
     *
     * @param  array  $attributes
     * @param  bool  $exists
     * @return \Illuminate\Database\Eloquent\Relations\Pivot
     */
    public function newPivot(array $attributes = [], $exists = false)
    {}

    /**
     * Create a new existing pivot model instance.
     *
     * @param  array  $attributes
     * @return \Illuminate\Database\Eloquent\Relations\Pivot
     */
    public function newExistingPivot(array $attributes = [])
    {}

    /**
     * Get a new plain query builder for the pivot table.
     *
     * @return \Illuminate\Database\Query\Builder
     */
    public function newPivotStatement()
    {}

    /**
     * Get a new pivot statement for a given "other" ID.
     *
     * @param  mixed  $id
     * @return \Illuminate\Database\Query\Builder
     */
    public function newPivotStatementForId($id)
    {}

    /**
     * Create a new query builder for the pivot table.
     *
     * @return \Illuminate\Database\Query\Builder
     */
    public function newPivotQuery()
    {}

    /**
     * Set the columns on the pivot table to retrieve.
     *
     * @param  mixed  $columns
     * @return $this
     */
    public function withPivot($columns)
    {}

    /**
     * Get all of the IDs from the given mixed value.
     *
     * @param  mixed  $value
     * @return array
     */
    protected function parseIds($value)
    {}

    /**
     * Get the ID from the given mixed value.
     *
     * @param  mixed  $value
     * @return mixed
     */
    protected function parseId($value)
    {}

    /**
     * Cast the given keys to integers if they are numeric and string otherwise.
     *
     * @param  array  $keys
     * @return array
     */
    protected function castKeys(array $keys)
    {}

    /**
     * Cast the given key to convert to primary key type.
     *
     * @param  mixed  $key
     * @return mixed
     */
    protected function castKey($key)
    {}

    /**
     * Cast the given pivot attributes.
     *
     * @param  array  $attributes
     * @return array
     */
    protected function castAttributes($attributes)
    {}

    /**
     * Converts a given value to a given type value.
     *
     * @param  string  $type
     * @param  mixed  $value
     * @return mixed
     */
    protected function getTypeSwapValue($type, $value)
    {}
}
