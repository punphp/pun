<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use Psr\Http\Message\UriInterface;

/**
 * Provides methods to determine if a modified URI should be considered
 * cross-origin.
 *
 * @author Graham Campbell
 */
final class UriComparator
{
    /**
     * Determines if a modified URI should be considered cross-origin with
     * respect to an original URI.
     *
     * Two URIs are cross-origin when their scheme, host, or effective port
     * differ. Host comparison is case-insensitive, and bracketed IPv6 literals
     * are canonicalized to their RFC 5952 form from any PSR-7 implementation
     * before comparison, so equivalent spellings of the same address are
     * same-origin. IPvFuture literals and bracketed values that cannot be
     * parsed as an IPv6 address, such as those carrying zone identifiers,
     * still compare as case-insensitive text. Missing ports use the default
     * port for `http`, `https`, `ws`, or `wss`. Other schemes do not receive
     * implicit default ports.
     *
     * This helper only compares URI origins. It does not implement redirect
     * handling or credential policy.
     */
    public static function isCrossOrigin(UriInterface $original, UriInterface $modified): bool
    {}

    private static function normalizeHost(UriInterface $uri): string
    {}

    private static function computePort(UriInterface $uri): ?int
    {}

    private function __construct()
    {}
}
