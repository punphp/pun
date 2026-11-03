<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

/**
 * @internal
 */
final class Integers
{
    private function __construct()
    {}

    public static function add(int $a, int $b): int
    {}

    public static function addSigned(int $base, int $delta): int
    {}

    /**
     * @param mixed $value
     */
    public static function assertEngineInteger($value, string $what): ?int
    {}

    /**
     * @param mixed $value
     */
    public static function assertOptionalNonNegativeSize($value, string $name): ?int
    {}

    /**
     * @param mixed $value
     */
    public static function assertNonNegativeInteger($value, string $name): int
    {}

    /**
     * @param mixed $value
     */
    public static function assertLimitInteger($value, string $name): int
    {}
}
