<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use InvalidArgumentException;
use Psr\Http\Message\ServerRequestInterface;
use Psr\Http\Message\UriInterface;

/**
 * @internal
 */
final class ServerRequestGlobalsFactory
{
    private function __construct()
    {}

    /**
     * @param array<array-key, mixed>  $server         Typically the $_SERVER superglobal
     * @param array<array-key, mixed>  $query          Typically the $_GET superglobal
     * @param array<array-key, mixed>  $post           Typically the $_POST superglobal
     * @param array<array-key, mixed>  $cookies        Typically the $_COOKIE superglobal
     * @param array<array-key, mixed>  $files          Typically the $_FILES superglobal
     * @param (callable(): mixed)|null $headerProvider
     */
    public static function fromArrays(
        #[\SensitiveParameter]
        array $server,
        array $query,
        array $post,
        #[\SensitiveParameter]
        array $cookies,
        array $files,
        ?callable $headerProvider = null
    ): ServerRequestInterface {}

    /**
     * @param array<array-key, mixed> $server Typically the $_SERVER superglobal
     */
    public static function getUriFromServerParams(
        #[\SensitiveParameter]
        array $server
    ): UriInterface {}

    /**
     * @param array<array-key, mixed>  $server
     * @param (callable(): mixed)|null $headerProvider
     *
     * @return array<array-key, string>
     */
    private static function getAllHeaders(
        #[\SensitiveParameter]
        array $server,
        ?callable $headerProvider
    ): array {}

    /**
     * @param array<array-key, mixed> $headers
     *
     * @return array<array-key, string>
     */
    private static function normalizeHeaderValues(
        #[\SensitiveParameter]
        array $headers
    ): array {}

    /**
     * @param array<array-key, mixed> $server Typically the $_SERVER superglobal
     *
     * @return array<array-key, string>
     */
    private static function getHeadersFromServer(array $server): array
    {}

    /**
     * @param array<array-key, string> $headers
     *
     * @return array<array-key, string>
     */
    private static function removeInvalidHostHeader(array $headers): array
    {}

    /**
     * @param array<array-key, mixed> $server
     */
    private static function getServerParam(array $server, string $key): ?string
    {}

    /**
     * @param array<array-key, mixed> $server
     */
    private static function getRequestMethodFromServer(array $server): string
    {}

    /**
     * @param array<array-key, mixed> $server
     */
    private static function getProtocolFromServer(array $server): string
    {}

    /**
     * @return array{0: string|null, 1: int|null}
     */
    private static function extractHostAndPortFromAuthority(string $authority): array
    {}

    private static function parseServerPort(string $port): int
    {}

    private static function withHostFromServer(UriInterface $uri, ?string $host): ?UriInterface
    {}

    /**
     * @param array<array-key, mixed> $server
     */
    private static function getUriWithSchemeFromServer(array $server): UriInterface
    {}

    /**
     * @param array<array-key, mixed> $server
     */
    private static function getAuthorityUriFromServer(
        #[\SensitiveParameter]
        array $server
    ): UriInterface {}

    /**
     * @param array<array-key, mixed> $server
     *
     * @return array{0: UriInterface, 1: string|null}
     */
    private static function getUriAndRequestTargetFromServer(
        #[\SensitiveParameter]
        array $server,
        string $method
    ): array {}

    /**
     * @return array{0: UriInterface, 1: string}|null
     */
    private static function getAbsoluteFormUriAndRequestTarget(
        #[\SensitiveParameter]
        string $requestUri,
        ?string $queryString
    ): ?array {}

    private static function removeUserInfoFromAbsoluteFormRequestTarget(string $target): string
    {}

    private static function hasEmptyPortInAbsoluteFormRequestTarget(string $target): bool
    {}

    /**
     * @return array{0: string, 1: int}|null
     */
    private static function parseConnectAuthorityFormRequestTarget(string $method, string $target): ?array
    {}

    private static function removeRequestTargetFragment(string $target): string
    {}

    /**
     * @return array{0: string, 1: string, 2: bool}
     */
    private static function splitRequestTargetQuery(string $target): array
    {}

    private static function normalizeOriginFormPathFromServer(string $path): string
    {}
}
