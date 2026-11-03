<?php

declare(strict_types=1);

namespace Brick\Math\Internal;

use Brick\Math\Exception\IntegerOverflowException;

use function is_int;
use function sprintf;

use const PHP_INT_MIN;

/**
 * Helpers for arithmetic operations that throw on native integer overflow.
 *
 * @internal
 */
final class Safe
{
    /**
     * @codeCoverageIgnore
     */
    private function __construct()
    {}

    /**
     * @pure
     */
    public static function add(int $a, int $b): int
    {}

    /**
     * @pure
     */
    public static function sub(int $a, int $b): int
    {}

    /**
     * @pure
     */
    public static function mul(int $a, int $b): int
    {}

    /**
     * @pure
     */
    public static function neg(int $value): int
    {}
}
