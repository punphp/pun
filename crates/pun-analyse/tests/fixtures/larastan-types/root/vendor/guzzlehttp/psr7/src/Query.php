<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

final class Query
{
    private function __construct()
    {}

    /**
     * Parse a query string into an associative array.
     *
     * If multiple values are found for the same key, the value of that
     * key-value pair becomes an array. This function does not parse nested PHP
     * style arrays into an associative array. For example, `foo[a]=1&foo[b]=2`
     * will be parsed into `['foo[a]' => '1', 'foo[b]' => '2']`.
     *
     * @param string   $str         Query string to parse
     * @param int|bool $urlEncoding How the query string is encoded
     */
    public static function parse(string $str, $urlEncoding = true): array
    {}

    /**
     * Build a query string from an array of key-value pairs.
     *
     * This function can use the return value of `parse()` to build a query
     * string. This function does not modify the provided keys when an array is
     * encountered, unlike `http_build_query()`.
     *
     * @param array     $params           Query string parameters.
     * @param int|false $encoding         Set to false to not encode,
     *                                    PHP_QUERY_RFC3986 to encode using
     *                                    RFC3986, or PHP_QUERY_RFC1738 to
     *                                    encode using RFC1738.
     * @param bool      $treatBoolsAsInts Set to true to encode as 0/1, and
     *                                    false as false/true.
     */
    public static function build(array $params, $encoding = PHP_QUERY_RFC3986, bool $treatBoolsAsInts = true): string
    {}

    /**
     * @param mixed $value
     */
    private static function normalizeValue($value, bool $treatBoolsAsInts): ?string
    {}
}
