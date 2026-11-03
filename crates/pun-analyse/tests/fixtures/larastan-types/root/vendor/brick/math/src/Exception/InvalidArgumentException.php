<?php

declare(strict_types=1);

namespace Brick\Math\Exception;

use function sprintf;

/**
 * Exception thrown when an invalid argument is provided.
 */
final class InvalidArgumentException extends \InvalidArgumentException implements MathException
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
    public static function baseOutOfRange(int $base): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function negativeScale(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function negativeBitIndex(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function negativeBitCount(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function alphabetTooShort(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function duplicateCharsInAlphabet(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function minGreaterThanMax(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function cannotConvertFloat(string $type): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function negativeExponent(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function negativeModulus(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function nonPositiveNthRootDegree(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function nonPositiveMaxDigits(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function invalidAllowedSyntax(): self
    {}
}
