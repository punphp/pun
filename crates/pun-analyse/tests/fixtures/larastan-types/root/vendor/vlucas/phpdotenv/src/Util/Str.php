<?php

declare(strict_types=1);

namespace Dotenv\Util;

use GrahamCampbell\ResultType\Error;
use GrahamCampbell\ResultType\Success;
use PhpOption\Option;

/**
 * @internal
 */
final class Str
{
    /**
     * This class is a singleton.
     *
     * @codeCoverageIgnore
     *
     * @return void
     */
    private function __construct()
    {}

    /**
     * Convert a string to UTF-8 from the given encoding.
     *
     * @param string      $input
     * @param string|null $encoding
     *
     * @return \GrahamCampbell\ResultType\Result<string, string>
     */
    public static function utf8(string $input, ?string $encoding = null)
    {}

    /**
     * Is the given character encoding valid?
     *
     * @param string $encoding
     *
     * @return bool
     */
    private static function isValidEncoding(string $encoding)
    {}

    /**
     * Search for a given substring of the input.
     *
     * @param string $haystack
     * @param string $needle
     *
     * @return \PhpOption\Option<int<0, max>>
     */
    public static function pos(string $haystack, string $needle)
    {}

    /**
     * Grab the specified substring of the input.
     *
     * @param string   $input
     * @param int      $start
     * @param int|null $length
     *
     * @return string
     */
    public static function substr(string $input, int $start, ?int $length = null)
    {}

    /**
     * Compute the length of the given string.
     *
     * @param string $input
     *
     * @return int
     */
    public static function len(string $input)
    {}
}
