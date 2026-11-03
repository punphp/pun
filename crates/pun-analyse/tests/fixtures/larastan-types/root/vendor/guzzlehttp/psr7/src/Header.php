<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

final class Header
{
    private function __construct()
    {}

    /**
     * Parses semicolon-separated header parameters into associative arrays, one
     * per comma-separated header value. Parameters without a value are appended
     * as values under integer keys.
     *
     * @param string|array $header Header to parse into components.
     */
    public static function parse($header): array
    {}

    /**
     * Split a header value into semicolon-separated parameters.
     *
     * @return string[]
     */
    private static function splitParameters(string $value): array
    {}

    /**
     * Splits an HTTP header defined to contain a comma-separated list into each
     * individual value. Empty values are removed.
     *
     * Example headers include `accept`, `cache-control`, and `if-none-match`.
     *
     * This method must not be used to parse headers that are not defined as a
     * list, such as `user-agent` or `set-cookie`.
     *
     * @param string|string[] $values Header value as returned by
     *                                MessageInterface::getHeader()
     *
     * @return string[]
     */
    public static function splitList($values): array
    {}
}
