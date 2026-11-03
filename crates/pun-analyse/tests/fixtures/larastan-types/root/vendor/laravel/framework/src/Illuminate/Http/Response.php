<?php

namespace Illuminate\Http;

use ArrayObject;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Contracts\Support\Jsonable;
use Illuminate\Contracts\Support\Renderable;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;
use JsonSerializable;
use Symfony\Component\HttpFoundation\Response as SymfonyResponse;
use Symfony\Component\HttpFoundation\ResponseHeaderBag;

class Response extends SymfonyResponse
{
    use ResponseTrait, Macroable {
        Macroable::__call as macroCall;
    }

    /**
     * Create a new HTTP response.
     *
     * @param  mixed  $content
     * @param  int  $status
     * @param  array  $headers
     *
     * @throws \InvalidArgumentException
     */
    public function __construct($content = '', $status = 200, array $headers = [])
    {}

    /**
     * Get the response content.
     */
    #[\Override]
    public function getContent(): string|false
    {}

    /**
     * Set the content on the response.
     *
     * @param  mixed  $content
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    #[\Override]
    public function setContent(mixed $content): static
    {}

    /**
     * Determine if the given content should be turned into JSON.
     *
     * @param  mixed  $content
     * @return bool
     */
    protected function shouldBeJson($content)
    {}

    /**
     * Morph the given content into JSON.
     *
     * @param  mixed  $content
     * @return string|false
     */
    protected function morphToJson($content)
    {}
}
