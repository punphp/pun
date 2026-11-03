<?php

namespace Illuminate\Database\Eloquent\Concerns;

use Illuminate\Database\Eloquent\ModelNotFoundException;

trait HasUniqueStringIds
{
    /**
     * Generate a new unique key for the model.
     *
     * @return mixed
     */
    abstract public function newUniqueId();

    /**
     * Determine if given key is valid.
     *
     * @param  mixed  $value
     * @return bool
     */
    abstract protected function isValidUniqueId($value): bool;

    /**
     * Initialize the trait.
     *
     * @return void
     */
    public function initializeHasUniqueStringIds()
    {}

    /**
     * Get the columns that should receive a unique identifier.
     *
     * @return array
     */
    public function uniqueIds()
    {}

    /**
     * Retrieve the model for a bound value.
     *
     * @param  \Illuminate\Database\Eloquent\Model|\Illuminate\Database\Eloquent\Relations\Relation<*, *, *>  $query
     * @param  mixed  $value
     * @param  string|null  $field
     * @return \Illuminate\Contracts\Database\Eloquent\Builder
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException
     */
    public function resolveRouteBindingQuery($query, $value, $field = null)
    {}

    /**
     * Get the auto-incrementing key type.
     *
     * @return string
     */
    public function getKeyType()
    {}

    /**
     * Get the value indicating whether the IDs are incrementing.
     *
     * @return bool
     */
    public function getIncrementing()
    {}

    /**
     * Throw an exception for the given invalid unique ID.
     *
     * @param  mixed  $value
     * @param  string|null  $field
     * @return never
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException
     */
    protected function handleInvalidUniqueId($value, $field)
    {}
}
