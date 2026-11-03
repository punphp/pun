<?php

declare(strict_types=1);

namespace GuzzleHttp;

use GuzzleHttp\Exception\BadResponseException;
use GuzzleHttp\Exception\InvalidArgumentException;
use GuzzleHttp\Exception\ResponseException;
use GuzzleHttp\Exception\TooManyRedirectsException;
use GuzzleHttp\Promise\PromiseInterface;
use GuzzleHttp\Psr7\HttpFactory;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;
use Psr\Http\Message\StreamFactoryInterface;
use Psr\Http\Message\StreamInterface;
use Psr\Http\Message\UriFactoryInterface;
use Psr\Http\Message\UriInterface;

/**
 * Request redirect middleware.
 *
 * Apply this middleware like other middleware using
 * {@see Middleware::redirect()}.
 *
 * @final
 */
class RedirectMiddleware
{
    use NonSerializableTrait;

    public const HISTORY_HEADER = 'X-Guzzle-Redirect-History';

    public const STATUS_HISTORY_HEADER = 'X-Guzzle-Redirect-Status-History';

    /**
     * @var array
     */
    public const DEFAULT_SETTINGS = [
        'max' => 5,
        'protocols' => ['http', 'https'],
        'strict' => false,
        'referer' => false,
        'track_redirects' => false,
    ];

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
     * @return ResponseInterface|PromiseInterface<ResponseInterface, mixed>
     */
    public function checkRedirect(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options,
        #[\SensitiveParameter]
        ResponseInterface $response
    ) {}

    /**
     * Enable tracking on promise.
     *
     * @param PromiseInterface<ResponseInterface, mixed> $promise
     *
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    private function withTracking(
        PromiseInterface $promise,
        #[\SensitiveParameter]
        string $uri,
        int $statusCode
    ): PromiseInterface {}

    /**
     * Check for too many redirects.
     *
     * @throws TooManyRedirectsException Too many redirects.
     */
    private function guardMax(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        ResponseInterface $response,
        #[\SensitiveParameter]
        array &$options
    ): void {}

    public function modifyRequest(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options,
        #[\SensitiveParameter]
        ResponseInterface $response
    ): RequestInterface {}

    private static function isRedirectStatusCode(int $statusCode): bool
    {}

    /**
     * @return array{
     *     method?: string,
     *     body?: StreamInterface,
     *     remove_headers?: list<string>
     * }
     */
    private static function getRedirectRequestModifiers(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options,
        #[\SensitiveParameter]
        ResponseInterface $response
    ): array {}

    /**
     * Set the appropriate URL on the request based on the location header.
     */
    private static function redirectUri(
        UriFactoryInterface $uriFactory,
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        ResponseInterface $response,
        array $protocols
    ): UriInterface {}
}
