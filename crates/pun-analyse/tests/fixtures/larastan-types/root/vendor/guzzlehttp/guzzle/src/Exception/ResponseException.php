<?php

declare(strict_types=1);

namespace GuzzleHttp\Exception;

use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;

/**
 * Exception thrown when a request fails after response headers are received.
 */
class ResponseException extends RequestException
{
    private ResponseInterface $response;

    final public function __construct(
        string $message,
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        ResponseInterface $response,
        #[\SensitiveParameter]
        ?\Throwable $previous = null
    ) {}

    /**
     * Return a new exception instance with an updated response.
     *
     * @param \Throwable|null $previous Previous exception, or the current
     *                                  previous exception when omitted.
     *
     * @return static
     */
    public function withResponse(
        #[\SensitiveParameter]
        ResponseInterface $response,
        #[\SensitiveParameter]
        ?\Throwable $previous = null
    ): self {}

    /**
     * Get the associated response.
     */
    public function getResponse(): ResponseInterface
    {}
}
