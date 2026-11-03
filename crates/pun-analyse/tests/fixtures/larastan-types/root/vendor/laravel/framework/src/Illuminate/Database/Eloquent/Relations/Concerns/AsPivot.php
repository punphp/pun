<?php

namespace Illuminate\Database\Eloquent\Relations\Concerns;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\MorphPivot;
use Illuminate\Support\Str;

trait AsPivot
{
    /**
     * The parent model of the relationship.
     *
     * @var \Illuminate\Database\Eloquent\Model
     */
    public $pivotParent;

    /**
     * The related model of the relationship.
     *
     * @var \Illuminate\Database\Eloquent\Model
     */
    public $pivotRelated;

    /**
     * The name of the foreign key column.
     *
     * @var string
     */
    protected $foreignKey;

    /**
     * The name of the "other key" column.
     *
     * @var string
     */
    protected $relatedKey;

    /**
     * Create a new pivot model instance.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $parent
     * @param  array  $attributes
     * @param  string  $table
     * @param  bool  $exists
     * @return static
     */
    public static function fromAttributes(Model $parent, $attributes, $table, $exists = false)
    {}

    /**
     * Create a new pivot model from raw values returned from a query.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $parent
     * @param  array  $attributes
     * @param  string  $table
     * @param  bool  $exists
     * @return static
     */
    public static function fromRawAttributes(Model $parent, $attributes, $table, $exists = false)
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
     * Set the keys for a save update query.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<static>  $query
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    protected function setKeysForSaveQuery($query)
    {}

    /**
     * Delete the pivot model record from the database.
     *
     * @return int
     */
    public function delete()
    {}

    /**
     * Get the query builder for a delete operation on the pivot.
     *
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    protected function getDeleteQuery()
    {}

    /**
     * Get the table associated with the model.
     *
     * @return string
     */
    public function getTable()
    {}

    /**
     * Get the foreign key column name.
     *
     * @return string
     */
    public function getForeignKey()
    {}

    /**
     * Get the "related key" column name.
     *
     * @return string
     */
    public function getRelatedKey()
    {}

    /**
     * Get the "related key" column name.
     *
     * @return string
     */
    public function getOtherKey()
    {}

    /**
     * Set the key names for the pivot model instance.
     *
     * @param  string  $foreignKey
     * @param  string  $relatedKey
     * @return $this
     */
    public function setPivotKeys($foreignKey, $relatedKey)
    {}

    /**
     * Set the related model of the relationship.
     *
     * @param  \Illuminate\Database\Eloquent\Model|null  $related
     * @return $this
     */
    public function setRelatedModel(?Model $related = null)
    {}

    /**
     * Determine if the pivot model or given attributes has timestamp attributes.
     *
     * @param  array|null  $attributes
     * @return bool
     */
    public function hasTimestampAttributes($attributes = null)
    {}

    /**
     * Get the name of the "created at" column.
     *
     * @return string
     */
    public function getCreatedAtColumn()
    {}

    /**
     * Get the name of the "updated at" column.
     *
     * @return string
     */
    public function getUpdatedAtColumn()
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
     * @param  int[]|string[]|string  $ids
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    public function newQueryForRestoration($ids)
    {}

    /**
     * Get a new query to restore multiple models by their queueable IDs.
     *
     * @param  int[]|string[]  $ids
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    protected function newQueryForCollectionRestoration(array $ids)
    {}

    /**
     * Unset all the loaded relations for the instance.
     *
     * @return $this
     */
    public function unsetRelations()
    {}
}
