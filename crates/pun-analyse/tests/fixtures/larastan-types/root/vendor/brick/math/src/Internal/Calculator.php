<?php

declare(strict_types=1);

namespace Brick\Math\Internal;

use Brick\Math\RoundingMode;

use function chr;
use function intdiv;
use function ltrim;
use function ord;
use function str_repeat;
use function strlen;
use function strpos;
use function strrev;
use function strtolower;
use function substr;

/**
 * Performs basic operations on arbitrary size integers.
 *
 * Unless otherwise specified, all parameters must be validated as non-empty strings of digits,
 * without leading zero, and with an optional leading minus sign if the number is not zero.
 *
 * Any other parameter format will lead to undefined behaviour.
 * All methods must return strings respecting this format, unless specified otherwise.
 *
 * @internal
 */
abstract readonly class Calculator
{
    /**
     * The alphabet for converting from and to base 2 to 36, lowercase.
     */
    public const ALPHABET = '0123456789abcdefghijklmnopqrstuvwxyz';

    /**
     * Returns the absolute value of a number.
     *
     * @pure
     */
    final public function abs(string $n): string
    {}

    /**
     * Negates a number.
     *
     * @pure
     */
    final public function neg(string $n): string
    {}

    /**
     * Compares two numbers.
     *
     * Returns -1 if the first number is less than, 0 if equal to, 1 if greater than the second number.
     *
     * @return -1|0|1
     *
     * @pure
     */
    final public function cmp(string $a, string $b): int
    {}

    /**
     * Adds two numbers.
     *
     * @pure
     */
    abstract public function add(string $a, string $b): string;

    /**
     * Subtracts two numbers.
     *
     * @pure
     */
    abstract public function sub(string $a, string $b): string;

    /**
     * Multiplies two numbers.
     *
     * @pure
     */
    abstract public function mul(string $a, string $b): string;

    /**
     * Returns the quotient of the division of two numbers.
     *
     * @param string $a The dividend.
     * @param string $b The divisor, must not be zero.
     *
     * @return string The quotient.
     *
     * @pure
     */
    abstract public function divQ(string $a, string $b): string;

    /**
     * Returns the remainder of the division of two numbers.
     *
     * @param string $a The dividend.
     * @param string $b The divisor, must not be zero.
     *
     * @return string The remainder.
     *
     * @pure
     */
    abstract public function divR(string $a, string $b): string;

    /**
     * Returns the quotient and remainder of the division of two numbers.
     *
     * @param string $a The dividend.
     * @param string $b The divisor, must not be zero.
     *
     * @return array{string, string} An array containing the quotient and remainder.
     *
     * @pure
     */
    abstract public function divQR(string $a, string $b): array;

    /**
     * Exponentiates a number.
     *
     * @param string $a The base number.
     * @param int    $e The exponent, validated as a non-negative integer.
     *
     * @return string The power.
     *
     * @pure
     */
    abstract public function pow(string $a, int $e): string;

    /**
     * @param string $b The modulus; must not be zero.
     *
     * @pure
     */
    public function mod(string $a, string $b): string
    {}

    /**
     * Returns the modular multiplicative inverse of $x modulo $m.
     *
     * If $x has no multiplicative inverse mod m, this method must return null.
     *
     * This method can be overridden by the concrete implementation if the underlying library has built-in support.
     *
     * @param string $m The modulus; must not be negative or zero.
     *
     * @pure
     */
    public function modInverse(string $x, string $m): ?string
    {}

    /**
     * Raises a number into power with modulo.
     *
     * @param string $base The base number.
     * @param string $exp  The exponent; must be positive or zero.
     * @param string $mod  The modulus; must be strictly positive.
     *
     * @pure
     */
    abstract public function modPow(string $base, string $exp, string $mod): string;

    /**
     * Returns the greatest common divisor of the two numbers.
     *
     * This method can be overridden by the concrete implementation if the underlying library
     * has built-in support for GCD calculations.
     *
     * @return string The GCD, always positive, or zero if both arguments are zero.
     *
     * @pure
     */
    public function gcd(string $a, string $b): string
    {}

    /**
     * Returns the least common multiple of the two numbers.
     *
     * This method can be overridden by the concrete implementation if the underlying library
     * has built-in support for LCM calculations.
     *
     * @return string The LCM, always positive, or zero if at least one argument is zero.
     *
     * @pure
     */
    public function lcm(string $a, string $b): string
    {}

    /**
     * Returns the square root of the given number, rounded down.
     *
     * The result is the largest x such that x² ≤ n.
     * The input MUST NOT be negative.
     *
     * @pure
     */
    abstract public function sqrt(string $n): string;

    /**
     * Returns the integer nth root of the given number, truncated toward zero.
     *
     * If $n is non-negative, the result is the largest x such that x^$k ≤ $n (floor).
     * If $n is negative, $k MUST be odd, and the result is the negation of the floor root of |$n|
     * (i.e., truncation toward zero: the smallest x such that x^$k ≥ $n).
     *
     * The caller MUST guarantee that $k ≥ 1 and that $n is non-negative when $k is even.
     *
     * This method can be overridden by the concrete implementation if the underlying library
     * has built-in support for nth root calculations.
     *
     * @param string $n The number. May be negative only when $k is odd.
     * @param int    $k The root degree. Must be strictly positive.
     *
     * @pure
     */
    public function nthRoot(string $n, int $k): string
    {}

    /**
     * Converts a number from an arbitrary base.
     *
     * This method can be overridden by the concrete implementation if the underlying library
     * has built-in support for base conversion.
     *
     * @param string $number The number, positive or zero, non-empty, case-insensitively validated for the given base.
     * @param int    $base   The base of the number, validated from 2 to 36.
     *
     * @return string The converted number, following the Calculator conventions.
     *
     * @pure
     */
    public function fromBase(string $number, int $base): string
    {}

    /**
     * Converts a number to an arbitrary base.
     *
     * This method can be overridden by the concrete implementation if the underlying library
     * has built-in support for base conversion.
     *
     * @param string $number The number to convert, following the Calculator conventions.
     * @param int    $base   The base to convert to, validated from 2 to 36.
     *
     * @return string The converted number, lowercase.
     *
     * @pure
     */
    public function toBase(string $number, int $base): string
    {}

    /**
     * Converts a non-negative number in an arbitrary base using a custom alphabet, to base 10.
     *
     * @param string $number   The number to convert, validated as a non-empty string,
     *                         containing only chars in the given alphabet/base.
     * @param string $alphabet The alphabet that contains every digit, validated as 2 chars minimum.
     * @param int    $base     The base of the number, validated from 2 to alphabet length.
     *
     * @return string The number in base 10, following the Calculator conventions.
     *
     * @pure
     */
    final public function fromArbitraryBase(string $number, string $alphabet, int $base): string
    {}

    /**
     * Converts a non-negative number to an arbitrary base using a custom alphabet.
     *
     * @param string $number   The number to convert, positive or zero, following the Calculator conventions.
     * @param string $alphabet The alphabet that contains every digit, validated as 2 chars minimum.
     * @param int    $base     The base to convert to, validated from 2 to alphabet length.
     *
     * @return string The converted number in the given alphabet.
     *
     * @pure
     */
    final public function toArbitraryBase(string $number, string $alphabet, int $base): string
    {}

    /**
     * Performs a rounded division.
     *
     * When the remainder of the division is not zero, rounding is performed according to the rounding mode provided,
     * unless RoundingMode::Unnecessary is used, in which case the method returns null.
     *
     * @param string       $a            The dividend.
     * @param string       $b            The divisor, must not be zero.
     * @param RoundingMode $roundingMode The rounding mode.
     *
     * @pure
     */
    final public function divRound(string $a, string $b, RoundingMode $roundingMode): ?string
    {}

    /**
     * Calculates bitwise AND of two numbers.
     *
     * This method can be overridden by the concrete implementation if the underlying library
     * has built-in support for bitwise operations.
     *
     * @pure
     */
    public function and(string $a, string $b): string
    {}

    /**
     * Calculates bitwise OR of two numbers.
     *
     * This method can be overridden by the concrete implementation if the underlying library
     * has built-in support for bitwise operations.
     *
     * @pure
     */
    public function or(string $a, string $b): string
    {}

    /**
     * Calculates bitwise XOR of two numbers.
     *
     * This method can be overridden by the concrete implementation if the underlying library
     * has built-in support for bitwise operations.
     *
     * @pure
     */
    public function xor(string $a, string $b): string
    {}

    /**
     * Extracts the sign & digits of the operands.
     *
     * @return array{bool, bool, string, string} Whether $a and $b are negative, followed by their digits.
     *
     * @pure
     */
    final protected function init(string $a, string $b): array
    {}

    /**
     * @param string $a Must be non-negative.
     * @param string $b Must be non-negative.
     *
     * @return array{string, string} GCD, X
     *
     * @pure
     */
    private function gcdExtended(string $a, string $b): array
    {}

    /**
     * Performs a bitwise operation on a decimal number.
     *
     * @param 'and'|'or'|'xor' $operator The operator to use.
     * @param string           $a        The left operand.
     * @param string           $b        The right operand.
     *
     * @pure
     */
    private function bitwise(string $operator, string $a, string $b): string
    {}

    /**
     * @param string $number A positive, binary number.
     *
     * @pure
     */
    private function twosComplement(string $number): string
    {}

    /**
     * Converts a decimal number to a binary string.
     *
     * @param string $number The number to convert, positive or zero, only digits.
     *
     * @pure
     */
    private function toBinary(string $number): string
    {}

    /**
     * Returns the positive decimal representation of a binary number.
     *
     * @param string $bytes The bytes representing the number.
     *
     * @pure
     */
    private function toDecimal(string $bytes): string
    {}
}
