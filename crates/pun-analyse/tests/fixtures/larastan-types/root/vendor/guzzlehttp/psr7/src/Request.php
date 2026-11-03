<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use InvalidArgumentException;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\StreamInterface;
use Psr\Http\Message\UriInterface;

/**
 * PSR-7 request implementation.
 */
class Request implements RequestInterface
{
    use MessageTrait;

    private string $method;

    private ?string $requestTarget = null;

    private UriInterface $uri;

    /**
     * @param string                               $method  HTTP method
     * @param string|UriInterface                  $uri     URI
     * @param (string|string[])[]                  $headers Request headers
     * @param string|resource|StreamInterface|null $body    Request body
     * @param string                               $version Protocol version
     */
    public function __construct(
        string $method,
        $uri,
        array $headers = [],
        $body = null,
        string $version = '1.1'
    ) {}

    public function getRequestTarget(): string
    {}

    public function withRequestTarget(string $requestTarget): RequestInterface
    {}

    public function getMethod(): string
    {}

    public function withMethod(string $method): RequestInterface
    {}

    public function getUri(): UriInterface
    {}

    public function withUri(UriInterface $uri, bool $preserveHost = false): RequestInterface
    {}

    private function updateHostFromUri(): void
    {}

    private function getHostFromUri(UriInterface $uri): ?string
    {}

    private function setHostHeader(string $host): void
    {}

    private function assertMethod(string $method): void
    {}

    private static function getRequestTargetFromUri(UriInterface $uri): string
    {}

    private static function assertRequestTarget(string $requestTarget): void
    {}

    private static function normalizePathForOriginForm(string $path): string
    {}
}
