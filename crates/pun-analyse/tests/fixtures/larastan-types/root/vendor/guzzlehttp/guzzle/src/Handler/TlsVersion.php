<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\Exception\InvalidArgumentException;

/**
 * @internal
 */
final class TlsVersion
{
    /**
     * @param mixed $value
     */
    public static function ordinal(string $option, $value): int
    {}

    /**
     * @param mixed $min
     * @param mixed $max
     */
    public static function assertRange($min, $max): void
    {}

    /**
     * @param mixed $value
     */
    public static function streamProtocolVersion(string $option, $value): int
    {}
}
