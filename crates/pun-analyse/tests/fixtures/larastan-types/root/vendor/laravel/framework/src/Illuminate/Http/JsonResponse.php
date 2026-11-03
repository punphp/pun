<?php

namespace Illuminate\Http;

use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Contracts\Support\Jsonable;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;
use JsonSerializable;
use Symfony\Component\HttpFoundation\JsonResponse as BaseJsonResponse;

class JsonResponse extends BaseJsonResponse
{
    use ResponseTrait, Macroable {
        Macroable::__call as macroCall;
    }

    /**
     * Create a new JSON response instance.
     *
     * @param  mixed  $data
     * @param  int  $status
     * @param  array  $headers
     * @param  int  $options
     * @param  bool  $json
     */
    public function __construct($data = null, $status = 200, $headers = [], $options = 0, $json = false)
    {}

    /**
     * {@inheritdoc}
     *
     * @return static
     */
    #[\Override]
    public static function fromJsonString(?string $data = null, int $status = 200, array $headers = []): static
    {}

    /**
     * Sets the JSONP callback.
     *
     * @param  string|null  $callback
     * @return $this
     */
    public function withCallback($callback = null)
    {}

    /**
     * Get the decoded JSON data from the response.
     *
     * @param  bool  $assoc
     * @param  int  $depth
     * @return mixed
     */
    public function getData($assoc = false, $depth = 512)
    {}

    /**
     * {@inheritdoc}
     *
     * @return static
     *
     * @throws \InvalidArgumentException
     */
    #[\Override]
    public function setData($data = []): static
    {}

    /**
     * Determine if an error occurred during JSON encoding.
     *
     * @param  int  $jsonError
     * @return bool
     */
    protected function hasValidJson($jsonError)
    {}

    /**
     * {@inheritdoc}
     *
     * @return static
     */
    #[\Override]
    public function setEncodingOptions($options): static
    {}

    /**
     * Determine if a JSON encoding option is set.
     *
     * @param  int  $option
     * @return bool
     */
    public function hasEncodingOption($option)
    {}
}
