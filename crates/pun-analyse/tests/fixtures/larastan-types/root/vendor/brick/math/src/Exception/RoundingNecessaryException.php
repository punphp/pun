<?php

declare(strict_types=1);

namespace Brick\Math\Exception;

use RuntimeException;

/**
 * Exception thrown when a number cannot be represented at the requested scale without rounding.
 */
final class RoundingNecessaryException extends RuntimeException implements MathException
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
    public static function decimalScaleTooSmall(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function rationalScaleTooSmall(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function integerDivisionNotExact(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function decimalDivisionNotExact(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function decimalDivisionScaleTooSmall(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function integerSquareRootNotExact(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function decimalSquareRootNotExact(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function decimalSquareRootScaleTooSmall(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function integerNthRootNotExact(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function decimalNthRootNotExact(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function decimalNthRootScaleTooSmall(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function decimalNotConvertibleToInteger(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function rationalNotConvertibleToInteger(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function rationalNotConvertibleToDecimal(): self
    {}
}
