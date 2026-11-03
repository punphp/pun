<?php

namespace Illuminate\Database\Eloquent\Relations;

class MorphPivot extends Pivot
{
    /**
     * The type of the polymorphic relation.
     *
     * Explicitly define this so it's not included in saved attributes.
     *
     * @var string
     */
    protected $morphType;

    /**
     * The value of the polymorphic relation.
     *
     * Explicitly define this so it's not included in saved attributes.
     *
     * @var class-string|string
     */
    protected $morphClass;

    /**
     * Set the keys for a save update query.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<static>  $query
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    protected function setKeysForSaveQuery($query)
    {}

    /**
     * Set the keys for a select query.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<static>  $query
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    protected function setKeysForSelectQuery($query)
    {}

    /**
     * Delete the pivot model record from the database.
     *
     * @return int
     */
    public function delete()
    {}

    /**
     * Get the morph type for the pivot.
     *
     * @return string
     */
    public function getMorphType()
    {}

    /**
     * Set the morph type for the pivot.
     *
     * @param  string  $morphType
     * @return $this
     */
    public function setMorphType($morphType)
    {}

    /**
     * Set the morph class for the pivot.
     *
     * @param  class-string|string  $morphClass
     * @return \Illuminate\Database\Eloquent\Relations\MorphPivot
     */
    public function setMorphClass($morphClass)
    {}

    /**
     * Get the queueable identity for the entity.
     *
     * @return mixed
     */
    public function getQueueableId()
    {}

    /**
     * Get a new query to restore one or more models by their queueable IDs.
     *
     * @param  array|int  $ids
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    public function newQueryForRestoration($ids)
    {}

    /**
     * Get a new query to restore multiple models by their queueable IDs.
     *
     * @param  array  $ids
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    protected function newQueryForCollectionRestoration(array $ids)
    {}
}
