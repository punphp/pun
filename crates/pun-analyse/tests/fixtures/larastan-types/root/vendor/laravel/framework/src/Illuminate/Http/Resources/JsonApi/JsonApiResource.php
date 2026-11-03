<?php

namespace Illuminate\Http\Resources\JsonApi;

use BadMethodCallException;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Http\Resources\Json\JsonResource;
use Illuminate\Support\Arr;

class JsonApiResource extends JsonResource
{
    use Concerns\ResolvesJsonApiElements,
        Concerns\ResolvesJsonApiRequest;

    /**
     * The "data" wrapper that should be applied.
     *
     * @var string|null
     */
    public static $wrap = 'data';

    /**
     * The resource's "version" for JSON:API.
     *
     * @var array{version?: string, ext?: array, profile?: array, meta?: array}
     */
    public static $jsonApiInformation = [];

    /**
     * The resource's "links" for JSON:API.
     */
    protected array $jsonApiLinks = [];

    /**
     * The resource's "meta" for JSON:API.
     */
    protected array $jsonApiMeta = [];

    /**
     * Set the JSON:API version for the request.
     *
     * @return void
     */
    public static function configure(?string $version = null, array $ext = [], array $profile = [], array $meta = [])
    {}

    /**
     * Get the resource's ID.
     *
     * @return string|null
     */
    public function toId(Request $request)
    {}

    /**
     * Get the resource's type.
     *
     * @return string|null
     */
    public function toType(Request $request)
    {}

    /**
     * Transform the resource into an array.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return \Illuminate\Contracts\Support\Arrayable|\JsonSerializable|array
     */
    #[\Override]
    public function toAttributes(Request $request)
    {}

    /**
     * Get the resource's relationships.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return \Illuminate\Contracts\Support\Arrayable|array
     */
    public function toRelationships(Request $request)
    {}

    /**
     * Get the resource's links.
     *
     * @return array
     */
    public function toLinks(Request $request)
    {}

    /**
     * Get the resource's meta information.
     *
     * @return array
     */
    public function toMeta(Request $request)
    {}

    /**
     * Get any additional data that should be returned with the resource array.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return array
     */
    #[\Override]
    public function with($request)
    {}

    /**
     * Resolve the resource to an array.
     *
     * @param  \Illuminate\Http\Request|null  $request
     * @return array
     */
    #[\Override]
    public function resolve($request = null)
    {}

    /**
     * Resolve the resource data to an array.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return array
     */
    #[\Override]
    public function resolveResourceData(Request $request)
    {}

    /**
     * Customize the outgoing response for the resource.
     */
    #[\Override]
    public function withResponse(Request $request, JsonResponse $response): void
    {}

    /**
     * Create an HTTP response that represents the object.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return \Illuminate\Http\JsonResponse
     */
    #[\Override]
    public function toResponse($request)
    {}

    /**
     * Resolve the HTTP request instance from container.
     *
     * @return \Illuminate\Http\Resources\JsonApi\JsonApiRequest
     */
    #[\Override]
    protected function resolveRequestFromContainer()
    {}

    /**
     * Create a new resource collection instance.
     *
     * @param  mixed  $resource
     * @return \Illuminate\Http\Resources\JsonApi\AnonymousResourceCollection
     */
    #[\Override]
    protected static function newCollection($resource)
    {}

    /**
     * Set the string that should wrap the outer-most resource array.
     *
     * @param  string  $value
     * @return never
     *
     * @throws \RuntimeException
     */
    #[\Override]
    public static function wrap($value)
    {}

    /**
     * Disable wrapping of the outer-most resource array.
     *
     * @return never
     */
    #[\Override]
    public static function withoutWrapping()
    {}

    /**
     * Flush the resource's global state.
     *
     * @return void
     */
    #[\Override]
    public static function flushState()
    {}
}
