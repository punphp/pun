<?php

declare(strict_types=1);

namespace Brick\Math\Exception;

use RuntimeException;

/**
 * Exception thrown when a division by zero occurs.
 */
final class DivisionByZeroException extends RuntimeException implements MathException
{
    /**
     * @internal
     *
     * @pure
     */
    public function __construct(string $message)
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function divisionByZero(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function zeroModulus(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function zeroDenominator(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function reciprocalOfZero(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function zeroToNegativePower(): self
    {}
}
