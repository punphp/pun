<?php

namespace Illuminate\Http\Resources\JsonApi\Concerns;

use Generator;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\Concerns\AsPivot;
use Illuminate\Database\Eloquent\Relations\Pivot;
use Illuminate\Database\Eloquent\Relations\Relation;
use Illuminate\Http\Resources\Json\JsonResource;
use Illuminate\Http\Resources\JsonApi\Exceptions\ResourceIdentificationException;
use Illuminate\Http\Resources\JsonApi\JsonApiRequest;
use Illuminate\Http\Resources\JsonApi\JsonApiResource;
use Illuminate\Http\Resources\JsonApi\RelationResolver;
use Illuminate\Http\Resources\MissingValue;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\LazyCollection;
use Illuminate\Support\Str;
use Illuminate\Support\Stringable;
use JsonSerializable;
use WeakMap;

trait ResolvesJsonApiElements
{
    /**
     * Determine whether resources respect inclusions and fields from the request.
     */
    protected bool $usesRequestQueryString = true;

    /**
     * Determine whether included relationship for the resource from eager loaded relationship.
     */
    protected bool $includesPreviouslyLoadedRelationships = false;

    /**
     * The relationships requested relative to this included resource.
     *
     * @var array<int, string>|null
     */
    protected ?array $requestedRelationships = null;

    /**
     * Cached loaded relationships map.
     *
     * @var array<int, array{0: \Illuminate\Http\Resources\JsonApi\JsonApiResource, 1: string, 2: string, 3: bool}>|null
     */
    public $loadedRelationshipsMap;

    /**
     * Cached loaded relationships identifiers.
     */
    protected array $loadedRelationshipIdentifiers = [];

    /**
     * The maximum relationship depth.
     */
    public static int $maxRelationshipDepth = 5;

    /**
     * Specify the maximum relationship depth.
     *
     * @param  non-negative-int  $depth
     */
    public static function maxRelationshipDepth(int $depth): void
    {}

    /**
     * Resolves `data` for the resource.
     */
    protected function resolveResourceObject(JsonApiRequest $request): array
    {}

    /**
     * Resolve the resource's identifier.
     *
     * @return string
     *
     * @throws ResourceIdentificationException
     */
    public function resolveResourceIdentifier(JsonApiRequest $request): string
    {}

    /**
     * Resolve the resource's type.
     *
     * @throws ResourceIdentificationException
     */
    public function resolveResourceType(JsonApiRequest $request): string
    {}

    /**
     * Resolve the resource's attributes.
     *
     * @throws \RuntimeException
     */
    protected function resolveResourceAttributes(JsonApiRequest $request, string $resourceType): array
    {}

    /**
     * Resolves `relationships` for the resource's data object.
     *
     * @return array
     *
     * @throws \RuntimeException
     */
    protected function resolveResourceRelationshipIdentifiers(JsonApiRequest $request): array
    {}

    /**
     * Compile resource relationships.
     */
    protected function compileResourceRelationships(JsonApiRequest $request): void
    {}

    /**
     * Compile resource relations.
     */
    protected function compileResourceRelationshipUsingResolver(
        JsonApiRequest $request,
        mixed $resource,
        RelationResolver $relationResolver,
        Collection|Model|null $relatedModels
    ): Generator { yield; }

    /**
     * Load the requested relationships that are missing on the given models at once.
     *
     * @internal
     */
    public function loadMissingRequestedRelationshipsOn(JsonApiRequest $request, Collection $models): void
    {}

    /**
     * Load the requested relationships that are missing on the given related models.
     */
    protected function loadMissingRequestedRelationships(
        JsonApiRequest $request,
        Collection $relatedModels,
        ?string $resourceClass,
        array $requestedRelationships
    ): void {}

    /**
     * Get the requested relationships for this resource or one of its relationships.
     */
    protected function requestedResourceRelationships(JsonApiRequest $request, ?string $relationName = null): array
    {}

    /**
     * Resolves `included` for the resource.
     */
    public function resolveIncludedResourceObjects(JsonApiRequest $request): Collection
    {}

    /**
     * Resolve the links for the resource.
     *
     * @return array<string, mixed>
     */
    protected function resolveResourceLinks(JsonApiRequest $request): array
    {}

    /**
     * Resolve the meta information for the resource.
     *
     * @return array<string, mixed>
     */
    protected function resolveResourceMetaInformation(JsonApiRequest $request): array
    {}

    /**
     * Indicate that relationship loading should respect the request's "includes" query string.
     *
     * @return $this
     */
    public function respectFieldsAndIncludesInQueryString(bool $value = true)
    {}

    /**
     * Indicate that relationship loading should not rely on the request's "includes" query string.
     *
     * @return $this
     */
    public function ignoreFieldsAndIncludesInQueryString()
    {}

    /**
     * Determine relationship should include loaded relationships.
     *
     * @return $this
     */
    public function includePreviouslyLoadedRelationships()
    {}
}
