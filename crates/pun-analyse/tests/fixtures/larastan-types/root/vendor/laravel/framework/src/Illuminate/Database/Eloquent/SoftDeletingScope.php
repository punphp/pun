<?php

namespace Illuminate\Database\Eloquent;

/**
 * @template TModel of \Illuminate\Database\Eloquent\Model
 *
 * @implements \Illuminate\Database\Eloquent\Scope<TModel>
 */
class SoftDeletingScope implements Scope
{
    /**
     * All of the extensions to be added to the builder.
     *
     * @var string[]
     */
    protected $extensions = ['Restore', 'RestoreOrCreate', 'CreateOrRestore', 'WithTrashed', 'WithoutTrashed', 'OnlyTrashed'];

    /**
     * Apply the scope to a given Eloquent query builder.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TModel>  $builder
     * @param  TModel  $model
     * @return void
     */
    public function apply(Builder $builder, Model $model)
    {}

    /**
     * Extend the query builder with the needed functions.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<*>  $builder
     * @return void
     */
    public function extend(Builder $builder)
    {}

    /**
     * Get the "deleted at" column for the builder.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<*>  $builder
     * @return string
     */
    protected function getDeletedAtColumn(Builder $builder)
    {}

    /**
     * Add the restore extension to the builder.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<*>  $builder
     * @return void
     */
    protected function addRestore(Builder $builder)
    {}

    /**
     * Add the restore-or-create extension to the builder.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<*>  $builder
     * @return void
     */
    protected function addRestoreOrCreate(Builder $builder)
    {}

    /**
     * Add the create-or-restore extension to the builder.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<*>  $builder
     * @return void
     */
    protected function addCreateOrRestore(Builder $builder)
    {}

    /**
     * Add the with-trashed extension to the builder.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<*>  $builder
     * @return void
     */
    protected function addWithTrashed(Builder $builder)
    {}

    /**
     * Add the without-trashed extension to the builder.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<*>  $builder
     * @return void
     */
    protected function addWithoutTrashed(Builder $builder)
    {}

    /**
     * Add the only-trashed extension to the builder.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<*>  $builder
     * @return void
     */
    protected function addOnlyTrashed(Builder $builder)
    {}
}
