<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\Psr7;
use GuzzleHttp\Utils;

/**
 * @internal
 */
final class HeaderProcessor
{
    private function __construct()
    {}

    /**
     * Returns the HTTP version, status code, reason phrase, and headers.
     *
     * @param string[] $headers
     *
     * @return array{0:string, 1:int, 2:?string, 3:array}
     *
     * @throws \RuntimeException
     */
    public static function parseHeaders(array $headers): array
    {}

    public static function isStatusLineCandidate(string $line): bool
    {}

    public static function isValidHeaderFieldLine(string $line): bool
    {}

    /**
     * Returns a normalized decimal Content-Length, or null when absent.
     *
     * @param string[] $values
     *
     * @throws \RuntimeException when Content-Length is malformed or conflicting.
     */
    public static function parseContentLength(array $values): ?string
    {}

    public static function contentLengthToInt(?string $length): ?int
    {}

    public static function assertContentLengthWithinPlatformLimit(?string $length): void
    {}

    /**
     * Validates response framing and returns its normalized Content-Length.
     * Returns null when absent or when ordinary body framing does not apply.
     *
     * @param array<string, string[]> $headers
     *
     * @throws \RuntimeException when Content-Length is malformed, conflicting,
     *                           or combined with Transfer-Encoding, or when a
     *                           transfer coding is unsupported or follows
     *                           chunked on the same field line
     */
    public static function validateResponseFraming(
        string $method,
        int $status,
        array $headers
    ): ?string {}

    /**
     * Returns whether the response uses the chunked transfer coding.
     *
     * Guzzle never asks for a transfer coding, so like libcurl 8.14 and newer
     * it accepts only identity and chunked, with nothing but chunked after
     * chunked on the same field line. Empty list elements are ignored.
     *
     * @param array<string, string[]> $headers
     *
     * @throws \RuntimeException when a coding is unsupported or follows chunked
     *                           on the same field line
     */
    public static function parseTransferEncoding(array $headers): bool
    {}

    /**
     * Removes every case-insensitive occurrence of a header and returns all
     * removed values in their original field order.
     *
     * @param array<string, string[]> $headers
     *
     * @return string[] Removed values across all header-name casings
     */
    public static function removeHeader(string $name, array &$headers): array
    {}

    /**
     * Whether a response uses ordinary body framing. A response to HEAD, a
     * response with a 1xx, 204, or 304 status code, or a 2xx response to
     * CONNECT never has a body, whatever its framing headers claim. A 205
     * remains subject to framing even though its semantics require no content.
     */
    public static function responseCanHaveBody(string $method, int $status): bool
    {}

    /**
     * @param non-empty-list<string> $headers
     *
     * @return list<string>
     */
    private static function getLastHeaderBlock(array $headers): array
    {}
}
