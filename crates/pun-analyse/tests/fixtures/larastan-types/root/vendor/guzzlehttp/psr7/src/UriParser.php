<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

/**
 * @internal
 */
final class UriParser
{
    private function __construct()
    {}

    /**
     * UTF-8 aware \parse_url() replacement.
     *
     * The internal function produces broken output for non ASCII domain names
     * (IDN) when used with locales other than "C".
     *
     * On the other hand, cURL understands IDN correctly only when UTF-8 locale
     * is configured ("C.UTF-8", "en_US.UTF-8", etc.).
     *
     * @see https://bugs.php.net/bug.php?id=52923
     * @see https://www.php.net/manual/en/function.parse-url.php#114817
     * @see https://curl.se/libcurl/c/CURLOPT_URL.html#ENCODING
     *
     * @return array|false
     */
    public static function parse(string $url)
    {}

    private static function isPathNoSchemeReference(string $url): bool
    {}

    /**
     * @return array{path: string, query?: string, fragment?: string}
     */
    private static function parsePathNoSchemeReference(string $url): array
    {}
}
