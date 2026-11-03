<?php

namespace Illuminate\Http\Resources\JsonApi;

use Illuminate\Http\Request;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;

class JsonApiRequest extends Request
{
    /**
     * Cached sparse fieldset.
     */
    protected ?array $cachedSparseFields = null;

    /**
     * Cached sparse included.
     */
    protected ?array $cachedSparseIncluded = null;

    /**
     * Get the request's included fields.
     */
    public function sparseFields(string $key): array
    {}

    /**
     * Determine if a sparse fieldset was provided for the given resource type.
     */
    public function hasSparseFieldset(string $key): bool
    {}

    /**
     * Get the request's included relationships.
     */
    public function sparseIncluded(?string $key = null): ?array
    {}
}
