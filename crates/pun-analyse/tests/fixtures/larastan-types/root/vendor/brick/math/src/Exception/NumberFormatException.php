<?php

declare(strict_types=1);

namespace Brick\Math\Exception;

use Brick\Math\NumberSyntax;
use RuntimeException;

use function ord;
use function sprintf;
use function strlen;
use function substr;

/**
 * Exception thrown when attempting to create a number from a string with an invalid format.
 */
final class NumberFormatException extends RuntimeException implements MathException
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
    public static function invalidFormat(string $value): self
    {}

    /**
     * @internal
     *
     * @param string $char The failing character.
     *
     * @pure
     */
    public static function charNotInAlphabet(string $char): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function charNotValidInBase(string $char, int $base): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function emptyNumber(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function emptyByteString(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function exponentTooLarge(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function tooManyDigits(int $maxDigits): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function syntaxNotAllowed(NumberSyntax $syntax): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function zeroDenominator(): self
    {}

    /**
     * @pure
     */
    private static function truncateAndEscape(string $value): string
    {}

    /**
     * @pure
     */
    private static function escapeChar(string $char): string
    {}
}
