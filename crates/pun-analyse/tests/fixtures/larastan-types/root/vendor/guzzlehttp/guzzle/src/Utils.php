<?php

declare(strict_types=1);

namespace GuzzleHttp;

use GuzzleHttp\Exception\InvalidArgumentException;
use GuzzleHttp\Handler\CurlHandler;
use GuzzleHttp\Handler\CurlMultiHandler;
use GuzzleHttp\Handler\CurlShareHandleState;
use GuzzleHttp\Handler\CurlVersion;
use GuzzleHttp\Handler\Proxy;
use GuzzleHttp\Handler\StreamHandler;
use GuzzleHttp\Handler\StreamTlsSessionCache;
use GuzzleHttp\Promise\PromiseInterface;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;

final class Utils
{
    private function __construct()
    {}

    /**
     * Parses an array of header lines into an associative array of headers.
     *
     * @param iterable $lines Header lines array of strings in the following
     *                        format: "Name: Value"
     */
    public static function headersFromLines(iterable $lines): array
    {}

    /**
     * Returns a debug stream based on the provided variable.
     *
     * @param mixed $value Optional value
     *
     * @return resource
     */
    public static function debugResource($value = null)
    {}

    /**
     * Chooses and creates a default handler to use based on the environment.
     *
     * The returned handler is not wrapped by any default middlewares.
     *
     * @param array{transport_sharing?: mixed, max_host_connections?: mixed, max_total_connections?: mixed, multiplex?: mixed} $handlerOptions Handler constructor options.
     *
     * @return callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed> Returns the best handler for the given system.
     *
     * @throws \RuntimeException if no viable Handler is available.
     */
    public static function chooseHandler(array $handlerOptions = []): callable
    {}

    private static function isTransportSharingRequired(string $sharingMode): bool
    {}

    /**
     * @param array{max_host_connections?: mixed, max_total_connections?: mixed} $handlerOptions
     */
    private static function hasConnectionCapOptions(array $handlerOptions): bool
    {}

    /**
     * @param array{max_host_connections?: mixed, max_total_connections?: mixed, multiplex?: mixed} $handlerOptions
     *
     * @return (callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)|null
     */
    private static function createCurlHandler(string $sharingMode, array $handlerOptions): ?callable
    {}

    /**
     * @return array<string, mixed>
     */
    private static function createCurlHandlerOptions(string $sharingMode): array
    {}

    /**
     * @param array{max_host_connections?: mixed, max_total_connections?: mixed} $handlerOptions
     *
     * @return array{max_host_connections?: int, max_total_connections?: int}
     */
    private static function connectionCapOptions(array $handlerOptions): array
    {}

    /**
     * @param (callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)|null $handler
     * @param array{max_host_connections?: int, max_total_connections?: int}                                         $connectionCapOptions
     *
     * @return callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>
     */
    private static function addStreamHandler(?callable $handler, string $sharingMode, array $connectionCapOptions): callable
    {}

    /**
     * Get the default User-Agent string to use with Guzzle.
     */
    public static function defaultUserAgent(): string
    {}

    /**
     * Creates an associative array of lowercase header names to the actual
     * header casing.
     */
    public static function normalizeHeaderKeys(array $headers): array
    {}

    /**
     * @param mixed $protocols
     *
     * @return string[]
     *
     * @throws InvalidArgumentException
     */
    public static function normalizeProtocols($protocols): array
    {}
}
