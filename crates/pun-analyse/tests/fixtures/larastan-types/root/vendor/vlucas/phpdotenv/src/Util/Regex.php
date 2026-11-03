<?php

declare(strict_types=1);

namespace Dotenv\Util;

use GrahamCampbell\ResultType\Error;
use GrahamCampbell\ResultType\Success;

/**
 * @internal
 */
final class Regex
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
     * Perform a preg match, wrapping up the result.
     *
     * @param string $pattern
     * @param string $subject
     *
     * @return \GrahamCampbell\ResultType\Result<bool, string>
     */
    public static function matches(string $pattern, string $subject)
    {}

    /**
     * Perform a preg match all, wrapping up the result.
     *
     * @param string $pattern
     * @param string $subject
     *
     * @return \GrahamCampbell\ResultType\Result<int<0, max>, string>
     */
    public static function occurrences(string $pattern, string $subject)
    {}

    /**
     * Perform a preg replace callback, wrapping up the result.
     *
     * @param string                     $pattern
     * @param callable(string[]): string $callback
     * @param string                     $subject
     * @param int|null                   $limit
     *
     * @return \GrahamCampbell\ResultType\Result<string, string>
     */
    public static function replaceCallback(string $pattern, callable $callback, string $subject, ?int $limit = null)
    {}

    /**
     * Perform a preg split, wrapping up the result.
     *
     * @param string $pattern
     * @param string $subject
     *
     * @return \GrahamCampbell\ResultType\Result<string[], string>
     */
    public static function split(string $pattern, string $subject)
    {}

    /**
     * Wrap the result of a preg operation.
     *
     * @template V
     *
     * @param V $result
     *
     * @return \GrahamCampbell\ResultType\Result<V, string>
     */
    private static function wrap($result)
    {}
}
