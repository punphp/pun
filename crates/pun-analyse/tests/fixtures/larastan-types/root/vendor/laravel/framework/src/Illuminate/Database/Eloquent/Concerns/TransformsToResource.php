<?php

namespace Illuminate\Database\Eloquent\Concerns;

use Illuminate\Database\Eloquent\Attributes\UseResource;
use Illuminate\Http\Resources\Json\JsonResource;
use Illuminate\Support\Str;
use LogicException;
use ReflectionClass;

trait TransformsToResource
{
    /**
     * Create a new resource object for the given resource.
     *
     * @param  class-string<\Illuminate\Http\Resources\Json\JsonResource>|null  $resourceClass
     * @return \Illuminate\Http\Resources\Json\JsonResource
     */
    public function toResource(?string $resourceClass = null): JsonResource
    {}

    /**
     * Guess the resource class for the model.
     *
     * @return \Illuminate\Http\Resources\Json\JsonResource
     *
     * @throws \LogicException
     */
    protected function guessResource(): JsonResource
    {}

    /**
     * Guess the resource class name for the model.
     *
     * @return array{class-string<\Illuminate\Http\Resources\Json\JsonResource>, class-string<\Illuminate\Http\Resources\Json\JsonResource>}
     */
    public static function guessResourceName(): array
    {}

    /**
     * Get the resource class from the class attribute.
     *
     * @param  class-string<\Illuminate\Http\Resources\Json\JsonResource>  $class
     * @return class-string<*>|null
     */
    protected function resolveResourceFromAttribute(string $class): ?string
    {}
}
