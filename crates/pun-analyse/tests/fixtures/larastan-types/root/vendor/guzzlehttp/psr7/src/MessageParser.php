<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;

/**
 * @internal
 */
final class MessageParser
{
    private function __construct()
    {}

    public static function parseMessage(string $message): array
    {}

    public static function parseRequestUri(string $path, array $headers): string
    {}

    private static function normalizePathForOriginForm(string $path): string
    {}

    /**
     * @return array{0: string, 1: int|null}
     */
    private static function parseHostHeaderAuthority(string $authority): array
    {}

    private static function composeAuthority(string $host, ?int $port): string
    {}

    /**
     * @param array $headers Array of headers (each value an array).
     */
    private static function getHostFromHeaders(array $headers): ?string
    {}

    /**
     * @param array $headers Array of headers (each value an array).
     */
    private static function getSingleHostHeader(array $headers): ?string
    {}

    /**
     * @param array $headers Array of headers (each value an array).
     */
    private static function parseRequestAuthorityUri(array $headers): string
    {}

    public static function parseRequest(string $message): RequestInterface
    {}

    private static function parseAbsoluteFormRequestTarget(string $target): ?Uri
    {}

    private static function parseConnectAuthorityFormRequestTarget(string $method, string $target): ?Uri
    {}

    public static function parseResponse(string $message): ResponseInterface
    {}
}
