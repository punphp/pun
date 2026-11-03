<?php

namespace Illuminate\Database\Eloquent\Concerns;

use Closure;
use Illuminate\Database\Eloquent\Attributes\ScopedBy;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Scope;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use InvalidArgumentException;
use ReflectionAttribute;
use ReflectionClass;

trait HasGlobalScopes
{
    /**
     * Boot the has global scopes trait for a model.
     *
     * @return void
     */
    public static function bootHasGlobalScopes()
    {}

    /**
     * Resolve the global scope class names from the attributes.
     *
     * @return array
     */
    public static function resolveGlobalScopeAttributes()
    {}

    /**
     * Register a new global scope on the model.
     *
     * @param  \Illuminate\Database\Eloquent\Scope|(\Closure(\Illuminate\Database\Eloquent\Builder<static>): mixed)|string  $scope
     * @param  \Illuminate\Database\Eloquent\Scope|(\Closure(\Illuminate\Database\Eloquent\Builder<static>): mixed)|null  $implementation
     * @return mixed
     *
     * @throws \InvalidArgumentException
     */
    public static function addGlobalScope($scope, $implementation = null)
    {}

    /**
     * Register multiple global scopes on the model.
     *
     * @param  array  $scopes
     * @return void
     */
    public static function addGlobalScopes(array $scopes)
    {}

    /**
     * Determine if a model has a global scope.
     *
     * @param  \Illuminate\Database\Eloquent\Scope|string  $scope
     * @return bool
     */
    public static function hasGlobalScope($scope)
    {}

    /**
     * Get a global scope registered with the model.
     *
     * @param  \Illuminate\Database\Eloquent\Scope|string  $scope
     * @return \Illuminate\Database\Eloquent\Scope|(\Closure(\Illuminate\Database\Eloquent\Builder<static>): mixed)|null
     */
    public static function getGlobalScope($scope)
    {}

    /**
     * Get all of the global scopes that are currently registered.
     *
     * @return array
     */
    public static function getAllGlobalScopes()
    {}

    /**
     * Set the current global scopes.
     *
     * @param  array  $scopes
     * @return void
     */
    public static function setAllGlobalScopes($scopes)
    {}

    /**
     * Get the global scopes for this class instance.
     *
     * @return array
     */
    public function getGlobalScopes()
    {}
}
