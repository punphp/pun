<?php

namespace Illuminate\Http\Resources\JsonApi;

use Closure;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Database\Eloquent\Model;

/**
 * @internal
 */
class RelationResolver
{
    /**
     * The relation resolver.
     *
     * @var \Closure(mixed):(\Illuminate\Database\Eloquent\Collection|\Illuminate\Database\Eloquent\Model|\Illuminate\Http\Resources\JsonApi\JsonApiResource|\Illuminate\Http\Resources\JsonApi\AnonymousResourceCollection|null)
     */
    public Closure $relationResolver;

    /**
     * The relation resource class.
     *
     * @var class-string<\Illuminate\Http\Resources\JsonApi\JsonApiResource>|null
     */
    public ?string $relationResourceClass = null;

    /**
     * Construct a new resource relationship resolver.
     *
     * @param  \Closure(mixed):(\Illuminate\Database\Eloquent\Collection|\Illuminate\Database\Eloquent\Model|\Illuminate\Http\Resources\JsonApi\JsonApiResource|\Illuminate\Http\Resources\JsonApi\AnonymousResourceCollection|null)|class-string<\Illuminate\Http\Resources\JsonApi\JsonApiResource>|null  $resolver
     */
    public function __construct(public string $relationName, Closure|string|null $resolver = null)
    {}

    /**
     * Resolve the relation for a resource.
     */
    public function handle(mixed $resource): Collection|Model|null
    {}

    /**
     * Get the resource class.
     *
     * @return class-string<\Illuminate\Http\Resources\JsonApi\JsonApiResource>|null
     */
    public function resourceClass(): ?string
    {}
}
