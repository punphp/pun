<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

/**
 * @internal
 */
final class TransferByteCounter
{
    private function __construct()
    {}

    /**
     * @param mixed $value
     */
    public static function progressValueToInt($value): int
    {}

    public static function add(int $current, int $delta, string $message): int
    {}
}
