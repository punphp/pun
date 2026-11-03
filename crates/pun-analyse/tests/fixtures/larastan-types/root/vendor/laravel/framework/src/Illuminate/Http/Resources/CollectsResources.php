<?php

namespace Illuminate\Http\Resources;

use Illuminate\Http\Resources\Attributes\Collects;
use Illuminate\Http\Resources\Json\JsonResource;
use Illuminate\Pagination\AbstractCursorPaginator;
use Illuminate\Pagination\AbstractPaginator;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use LogicException;
use ReflectionClass;
use Traversable;

trait CollectsResources
{
    /**
     * The cached Collects attribute values.
     *
     * @var array<class-string, class-string|false>
     */
    protected static $cachedCollectsAttributes = [];

    /**
     * Map the given collection resource into its individual resources.
     *
     * @param  mixed  $resource
     * @return mixed
     */
    protected function collectResource($resource)
    {}

    /**
     * Get the resource that this resource collects.
     *
     * @return class-string<\Illuminate\Http\Resources\Json\JsonResource>|null
     *
     * @throws \LogicException
     */
    protected function collects()
    {}

    /**
     * Get the JSON serialization options that should be applied to the resource response.
     *
     * @return int
     *
     * @throws \ReflectionException
     */
    public function jsonOptions()
    {}

    /**
     * Get an iterator for the resource collection.
     *
     * @return \ArrayIterator
     */
    public function getIterator(): Traversable
    {}
}
