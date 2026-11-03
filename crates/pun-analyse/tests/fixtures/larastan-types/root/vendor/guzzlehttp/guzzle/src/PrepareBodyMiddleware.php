<?php

declare(strict_types=1);

namespace GuzzleHttp;

use GuzzleHttp\Handler\RequestFraming;
use GuzzleHttp\Promise\PromiseInterface;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;

/**
 * Prepares requests that contain a body, adding the Content-Length,
 * Content-Type, and Expect headers.
 *
 * @final
 */
class PrepareBodyMiddleware
{
    use NonSerializableTrait;

    /**
     * @var callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>
     */
    private $nextHandler;

    /**
     * @param callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed> $nextHandler Next handler to invoke.
     */
    public function __construct(callable $nextHandler)
    {}

    /**
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    public function __invoke(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options
    ): PromiseInterface {}

    /**
     * Add expect header
     */
    private function addExpectHeader(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options,
        array &$modify,
        ?int $bodySize
    ): void {}
}
