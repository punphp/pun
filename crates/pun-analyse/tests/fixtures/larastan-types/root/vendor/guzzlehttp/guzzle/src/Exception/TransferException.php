<?php

declare(strict_types=1);

namespace GuzzleHttp\Exception;

use Psr\Http\Message\RequestInterface;

/**
 * Base exception for transfer failures.
 */
class TransferException extends \RuntimeException implements GuzzleException
{
    private RequestInterface $request;

    public function __construct(
        string $message,
        RequestInterface $request,
        int $code = 0,
        ?\Throwable $previous = null
    ) {}

    /**
     * Get the request that caused the exception.
     */
    public function getRequest(): RequestInterface
    {}
}
