<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\Exception\InvalidArgumentException;

/**
 * @internal
 */
final class Timeout
{
    private function __construct()
    {}

    /**
     * Converts a request timeout option to integer milliseconds.
     *
     * @param mixed $value
     */
    public static function toMilliseconds($value, string $option): int
    {}
}
