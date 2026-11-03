<?php

namespace Illuminate\Support\Traits;

use Illuminate\Database\Eloquent\Attributes\UseResource;
use Illuminate\Database\Eloquent\Attributes\UseResourceCollection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Http\Resources\Json\ResourceCollection;
use LogicException;
use ReflectionClass;

trait TransformsToResourceCollection
{
    /**
     * Create a new resource collection instance for the given resource.
     *
     * @param  class-string<\Illuminate\Http\Resources\Json\JsonResource>|null  $resourceClass
     * @return \Illuminate\Http\Resources\Json\ResourceCollection
     *
     * @throws \Throwable
     */
    public function toResourceCollection(?string $resourceClass = null): ResourceCollection
    {}

    /**
     * Guess the resource collection for the items.
     *
     * @return \Illuminate\Http\Resources\Json\ResourceCollection
     *
     * @throws \Throwable
     */
    protected function guessResourceCollection(): ResourceCollection
    {}

    /**
     * Get the resource class from the class attribute.
     *
     * @param  class-string<\Illuminate\Http\Resources\Json\JsonResource>  $class
     * @return class-string<*>|null
     */
    protected function resolveResourceFromAttribute(string $class): ?string
    {}

    /**
     * Get the resource collection class from the class attribute.
     *
     * @param  class-string<\Illuminate\Http\Resources\Json\ResourceCollection>  $class
     * @return class-string<*>|null
     */
    protected function resolveResourceCollectionFromAttribute(string $class): ?string
    {}
}
