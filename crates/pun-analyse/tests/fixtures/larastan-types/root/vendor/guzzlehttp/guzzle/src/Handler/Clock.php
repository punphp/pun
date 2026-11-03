<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

/**
 * @internal
 */
final class Clock
{
    private function __construct()
    {}

    /**
     * Returns the current monotonic clock reading in seconds.
     */
    public static function now(): float
    {}

    public static function sleep(int $microseconds): void
    {}
}
