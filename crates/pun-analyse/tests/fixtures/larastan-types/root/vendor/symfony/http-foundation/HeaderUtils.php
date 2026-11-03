<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpFoundation;

/**
 * HTTP header utility functions.
 *
 * @author Christian Schmidt <github@chsc.dk>
 */
class HeaderUtils
{
    public const DISPOSITION_ATTACHMENT = 'attachment';
    public const DISPOSITION_INLINE = 'inline';

    /**
     * This class should not be instantiated.
     */
    private function __construct()
    {}

    /**
     * Splits an HTTP header by one or more separators.
     *
     * Example:
     *
     *     HeaderUtils::split('da, en-gb;q=0.8', ',;')
     *     # returns [['da'], ['en-gb', 'q=0.8']]
     *
     * @param string $separators List of characters to split on, ordered by
     *                           precedence, e.g. ',', ';=', or ',;='
     *
     * @return array Nested array with as many levels as there are characters in
     *               $separators
     */
    public static function split(string $header, string $separators): array
    {}

    /**
     * Combines an array of arrays into one associative array.
     *
     * Each of the nested arrays should have one or two elements. The first
     * value will be used as the keys in the associative array, and the second
     * will be used as the values, or true if the nested array only contains one
     * element. Array keys are lowercased.
     *
     * Example:
     *
     *     HeaderUtils::combine([['foo', 'abc'], ['bar']])
     *     // => ['foo' => 'abc', 'bar' => true]
     */
    public static function combine(array $parts): array
    {}

    /**
     * Joins an associative array into a string for use in an HTTP header.
     *
     * The key and value of each entry are joined with '=', and all entries
     * are joined with the specified separator and an additional space (for
     * readability). Values are quoted if necessary.
     *
     * Example:
     *
     *     HeaderUtils::toString(['foo' => 'abc', 'bar' => true, 'baz' => 'a b c'], ',')
     *     // => 'foo=abc, bar, baz="a b c"'
     */
    public static function toString(array $assoc, string $separator): string
    {}

    /**
     * Encodes a string as a quoted string, if necessary.
     *
     * If a string contains characters not allowed by the "token" construct in
     * the HTTP specification, it is backslash-escaped and enclosed in quotes
     * to match the "quoted-string" construct.
     */
    public static function quote(string $s): string
    {}

    /**
     * Decodes a quoted string.
     *
     * If passed an unquoted string that matches the "token" construct (as
     * defined in the HTTP specification), it is passed through verbatim.
     */
    public static function unquote(string $s): string
    {}

    /**
     * Generates an HTTP Content-Disposition field-value.
     *
     * @param string $disposition      One of "inline" or "attachment"
     * @param string $filename         A unicode string
     * @param string $filenameFallback A string containing only ASCII characters that
     *                                 is semantically equivalent to $filename. If the filename is already ASCII,
     *                                 it can be omitted, or just copied from $filename
     *
     * @throws \InvalidArgumentException
     *
     * @see RFC 6266
     */
    public static function makeDisposition(string $disposition, string $filename, string $filenameFallback = ''): string
    {}

    /**
     * Like parse_str(), but preserves dots in variable names.
     */
    public static function parseQuery(string $query, bool $ignoreBrackets = false, string $separator = '&'): array
    {}

    private static function groupParts(array $matches, string $separators, bool $first = true): array
    {}
}
