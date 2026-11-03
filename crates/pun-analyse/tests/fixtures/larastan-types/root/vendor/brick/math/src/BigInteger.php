<?php

declare(strict_types=1);

namespace Brick\Math;

use Brick\Math\Exception\DivisionByZeroException;
use Brick\Math\Exception\IntegerOverflowException;
use Brick\Math\Exception\InvalidArgumentException;
use Brick\Math\Exception\MathException;
use Brick\Math\Exception\NegativeNumberException;
use Brick\Math\Exception\NoInverseException;
use Brick\Math\Exception\NumberFormatException;
use Brick\Math\Exception\PlatformException;
use Brick\Math\Exception\RandomSourceException;
use Brick\Math\Exception\RoundingNecessaryException;
use Brick\Math\Internal\Calculator;
use Brick\Math\Internal\CalculatorRegistry;
use Brick\Math\Internal\Safe;
use LogicException;
use Override;
use Throwable;

use function array_map;
use function assert;
use function bin2hex;
use function chr;
use function count_chars;
use function filter_var;
use function hex2bin;
use function in_array;
use function intdiv;
use function is_string;
use function ltrim;
use function ord;
use function preg_match;
use function preg_quote;
use function random_bytes;
use function str_repeat;
use function strlen;
use function substr;

use const FILTER_VALIDATE_INT;

/**
 * An arbitrarily large integer number.
 *
 * This class is immutable.
 */
final readonly class BigInteger extends BigNumber
{
    /**
     * The value, as a string of digits with optional leading minus sign.
     *
     * No leading zeros must be present.
     * No leading minus sign must be present if the number is zero.
     */
    private string $value;

    /**
     * Protected constructor. Use a factory method to obtain an instance.
     *
     * @param string $value A string of digits, with optional leading minus sign.
     *
     * @pure
     */
    protected function __construct(string $value)
    {}

    /**
     * Creates a number from a string in a given base.
     *
     * The string can optionally be prefixed with the `+` or `-` sign.
     *
     * Bases greater than 36 are not supported by this method, as there is no clear consensus on which of the lowercase
     * or uppercase characters should come first. Instead, this method accepts any base up to 36, and does not
     * differentiate lowercase and uppercase characters, which are considered equal.
     *
     * For bases greater than 36, and/or custom alphabets, use the fromArbitraryBase() method.
     *
     * @param non-empty-string $number The number to convert, in the given base.
     * @param int<2, 36>       $base   The base of the number, between 2 and 36.
     *
     * @throws NumberFormatException    If the number is empty, or contains invalid chars for the given base.
     * @throws InvalidArgumentException If the base is out of range.
     *
     * @pure
     */
    public static function fromBase(string $number, int $base): BigInteger
    {}

    /**
     * Parses a string containing an integer in an arbitrary base, using a custom alphabet.
     *
     * This method is byte-oriented: the alphabet is interpreted as a sequence of single-byte characters.
     * Multibyte UTF-8 characters are not supported.
     *
     * Because this method accepts any single-byte character, including dash, it does not handle negative numbers.
     *
     * @param non-empty-string $number   The number to parse.
     * @param non-empty-string $alphabet The alphabet, for example '01' for base 2, or '01234567' for base 8.
     *
     * @throws NumberFormatException    If the given number is empty or contains invalid chars for the given alphabet.
     * @throws InvalidArgumentException If the alphabet does not contain at least 2 chars, or contains duplicates.
     *
     * @pure
     */
    public static function fromArbitraryBase(string $number, string $alphabet): BigInteger
    {}

    /**
     * Translates a string of bytes containing the binary representation of a BigInteger into a BigInteger.
     *
     * The input string is assumed to be in big-endian byte-order: the most significant byte is in the zeroth element.
     *
     * If `$signed` is true, the input is assumed to be in two's-complement representation, and the leading bit is
     * interpreted as a sign bit. If `$signed` is false, the input is interpreted as an unsigned number, and the
     * resulting BigInteger will always be positive or zero.
     *
     * This method can be used to retrieve a number exported by `toBytes()`, as long as the `$signed` flags match.
     *
     * @param non-empty-string $bytes  The byte string.
     * @param bool             $signed Whether to interpret as a signed number in two's-complement representation with a leading
     *                                 sign bit.
     *
     * @throws NumberFormatException If the string is empty.
     *
     * @pure
     */
    public static function fromBytes(string $bytes, bool $signed = true): BigInteger
    {}

    /**
     * Generates a pseudo-random number in the range 0 to 2^bitCount - 1.
     *
     * Using the default random bytes generator, this method is suitable for cryptographic use.
     *
     * @param non-negative-int             $bitCount             The number of bits.
     * @param (callable(int): string)|null $randomBytesGenerator A function that accepts a number of bytes, and returns
     *                                                           a string of random bytes of the given length. Defaults
     *                                                           to the `random_bytes()` function.
     *
     * @throws InvalidArgumentException If $bitCount is negative.
     * @throws RandomSourceException    If random byte generation fails.
     */
    public static function randomBits(int $bitCount, ?callable $randomBytesGenerator = null): BigInteger
    {}

    /**
     * Generates a pseudo-random number between `$min` and `$max`, inclusive.
     *
     * Using the default random bytes generator, this method is suitable for cryptographic use.
     *
     * @param BigNumber|int|string         $min                  The lower bound. Must be convertible to a BigInteger.
     * @param BigNumber|int|string         $max                  The upper bound. Must be convertible to a BigInteger.
     * @param (callable(int): string)|null $randomBytesGenerator A function that accepts a number of bytes, and returns
     *                                                           a string of random bytes of the given length. Defaults
     *                                                           to the `random_bytes()` function.
     *
     * @throws MathException            If one of the parameters cannot be converted to a BigInteger.
     * @throws InvalidArgumentException If `$min` is greater than `$max`.
     * @throws RandomSourceException    If random byte generation fails.
     */
    public static function randomRange(
        BigNumber|int|string $min,
        BigNumber|int|string $max,
        ?callable $randomBytesGenerator = null,
    ): BigInteger {}

    /**
     * Returns a BigInteger representing zero.
     *
     * @pure
     */
    public static function zero(): BigInteger
    {}

    /**
     * Returns a BigInteger representing one.
     *
     * @pure
     */
    public static function one(): BigInteger
    {}

    /**
     * Returns a BigInteger representing ten.
     *
     * @pure
     */
    public static function ten(): BigInteger
    {}

    /**
     * Returns the greatest common divisor of the given numbers.
     *
     * The GCD is always positive, unless all numbers are zero, in which case it is zero.
     *
     * @param BigNumber|int|string $a    The first number. Must be convertible to a BigInteger.
     * @param BigNumber|int|string ...$n The additional numbers. Each number must be convertible to a BigInteger.
     *
     * @throws MathException If one of the parameters cannot be converted to a BigInteger.
     *
     * @pure
     */
    public static function gcdAll(BigNumber|int|string $a, BigNumber|int|string ...$n): BigInteger
    {}

    /**
     * Returns the least common multiple of the given numbers.
     *
     * The LCM is always positive, unless one of the numbers is zero, in which case it is zero.
     *
     * @param BigNumber|int|string $a    The first number. Must be convertible to a BigInteger.
     * @param BigNumber|int|string ...$n The additional numbers. Each number must be convertible to a BigInteger.
     *
     * @throws MathException If one of the parameters cannot be converted to a BigInteger.
     *
     * @pure
     */
    public static function lcmAll(BigNumber|int|string $a, BigNumber|int|string ...$n): BigInteger
    {}

    /**
     * Returns the sum of this number and the given one.
     *
     * @param BigNumber|int|string $that The number to add. Must be convertible to a BigInteger.
     *
     * @throws MathException If the number is not valid, or is not convertible to a BigInteger.
     *
     * @pure
     */
    public function plus(BigNumber|int|string $that): BigInteger
    {}

    /**
     * Returns the difference of this number and the given one.
     *
     * @param BigNumber|int|string $that The number to subtract. Must be convertible to a BigInteger.
     *
     * @throws MathException If the number is not valid, or is not convertible to a BigInteger.
     *
     * @pure
     */
    public function minus(BigNumber|int|string $that): BigInteger
    {}

    /**
     * Returns the product of this number and the given one.
     *
     * @param BigNumber|int|string $that The multiplier. Must be convertible to a BigInteger.
     *
     * @throws MathException If the multiplier is not valid, or is not convertible to a BigInteger.
     *
     * @pure
     */
    public function multipliedBy(BigNumber|int|string $that): BigInteger
    {}

    /**
     * Returns the result of the division of this number by the given one.
     *
     * @param BigNumber|int|string $that         The divisor. Must be convertible to a BigInteger.
     * @param RoundingMode         $roundingMode An optional rounding mode, defaults to Unnecessary.
     *
     * @throws MathException              If the divisor is not valid, or is not convertible to a BigInteger.
     * @throws DivisionByZeroException    If the divisor is zero.
     * @throws RoundingNecessaryException If RoundingMode::Unnecessary is used and the remainder is not zero.
     *
     * @pure
     */
    public function dividedBy(BigNumber|int|string $that, RoundingMode $roundingMode = RoundingMode::Unnecessary): BigInteger
    {}

    /**
     * Returns this number exponentiated to the given value.
     *
     * @param non-negative-int $exponent
     *
     * @throws InvalidArgumentException If the exponent is negative.
     *
     * @pure
     */
    public function power(int $exponent): BigInteger
    {}

    /**
     * Returns the quotient of the division of this number by the given one.
     *
     * Examples:
     *
     * - `7` quotient `3` returns `2`
     * - `7` quotient `-3` returns `-2`
     * - `-7` quotient `3` returns `-2`
     * - `-7` quotient `-3` returns `2`
     *
     * @param BigNumber|int|string $that The divisor. Must be convertible to a BigInteger.
     *
     * @throws MathException           If the divisor is not valid, or is not convertible to a BigInteger.
     * @throws DivisionByZeroException If the divisor is zero.
     *
     * @pure
     */
    public function quotient(BigNumber|int|string $that): BigInteger
    {}

    /**
     * Returns the remainder of the division of this number by the given one.
     *
     * The remainder, when non-zero, has the same sign as the dividend.
     *
     * Examples:
     *
     * - `7` remainder `3` returns `1`
     * - `7` remainder `-3` returns `1`
     * - `-7` remainder `3` returns `-1`
     * - `-7` remainder `-3` returns `-1`
     *
     * @param BigNumber|int|string $that The divisor. Must be convertible to a BigInteger.
     *
     * @throws MathException           If the divisor is not valid, or is not convertible to a BigInteger.
     * @throws DivisionByZeroException If the divisor is zero.
     *
     * @pure
     */
    public function remainder(BigNumber|int|string $that): BigInteger
    {}

    /**
     * Returns the quotient and remainder of the division of this number by the given one.
     *
     * Examples:
     *
     * - `7` quotientAndRemainder `3` returns [`2`, `1`]
     * - `7` quotientAndRemainder `-3` returns [`-2`, `1`]
     * - `-7` quotientAndRemainder `3` returns [`-2`, `-1`]
     * - `-7` quotientAndRemainder `-3` returns [`2`, `-1`]
     *
     * @param BigNumber|int|string $that The divisor. Must be convertible to a BigInteger.
     *
     * @return array{BigInteger, BigInteger} An array containing the quotient and the remainder.
     *
     * @throws MathException           If the divisor is not valid, or is not convertible to a BigInteger.
     * @throws DivisionByZeroException If the divisor is zero.
     *
     * @pure
     */
    public function quotientAndRemainder(BigNumber|int|string $that): array
    {}

    /**
     * Returns this number modulo the given one.
     *
     * The result is always non-negative, and is the unique value `r` such that `0 <= r < m`
     * and `this - r` is a multiple of `m`.
     *
     * This is also known as Euclidean modulo. Unlike `remainder()`, which can return negative values
     * when the dividend is negative, `mod()` always returns a non-negative result.
     *
     * Examples:
     *
     * - `7` mod `3` returns `1`
     * - `-7` mod `3` returns `2`
     *
     * @param BigNumber|int|string $modulus The modulus. Must be convertible to a BigInteger.
     *
     * @throws MathException            If the modulus is not valid, or is not convertible to a BigInteger.
     * @throws InvalidArgumentException If the modulus is negative.
     * @throws DivisionByZeroException  If the modulus is zero.
     *
     * @pure
     */
    public function mod(BigNumber|int|string $modulus): BigInteger
    {}

    /**
     * Returns the modular multiplicative inverse of this BigInteger modulo $modulus.
     *
     * @param BigNumber|int|string $modulus The modulus. Must be convertible to a BigInteger.
     *
     * @throws MathException            If the modulus is not valid, or is not convertible to a BigInteger.
     * @throws InvalidArgumentException If the modulus is negative.
     * @throws DivisionByZeroException  If the modulus is zero.
     * @throws NoInverseException       If this BigInteger has no multiplicative inverse mod m (that is, this BigInteger
     *                                  is not relatively prime to m).
     *
     * @pure
     */
    public function modInverse(BigNumber|int|string $modulus): BigInteger
    {}

    /**
     * Returns this number raised into power with modulo.
     *
     * This operation requires a non-negative exponent and a strictly positive modulus.
     *
     * @param BigNumber|int|string $exponent The exponent. Must be convertible to a BigInteger.
     * @param BigNumber|int|string $modulus  The modulus. Must be convertible to a BigInteger.
     *
     * @throws MathException            If the exponent or modulus is not valid, or is not convertible to a BigInteger.
     * @throws InvalidArgumentException If the exponent or modulus is negative.
     * @throws DivisionByZeroException  If the modulus is zero.
     *
     * @pure
     */
    public function modPow(BigNumber|int|string $exponent, BigNumber|int|string $modulus): BigInteger
    {}

    /**
     * Returns the greatest common divisor of this number and the given one.
     *
     * The GCD is always positive, unless both operands are zero, in which case it is zero.
     *
     * @param BigNumber|int|string $that The operand. Must be convertible to a BigInteger.
     *
     * @throws MathException If the operand is not valid, or is not convertible to a BigInteger.
     *
     * @pure
     */
    public function gcd(BigNumber|int|string $that): BigInteger
    {}

    /**
     * Returns the least common multiple of this number and the given one.
     *
     * The LCM is always positive, unless at least one operand is zero, in which case it is zero.
     *
     * @param BigNumber|int|string $that The operand. Must be convertible to a BigInteger.
     *
     * @throws MathException If the operand is not valid, or is not convertible to a BigInteger.
     *
     * @pure
     */
    public function lcm(BigNumber|int|string $that): BigInteger
    {}

    /**
     * Returns the integer square root of this number, rounded according to the given rounding mode.
     *
     * @param RoundingMode $roundingMode An optional rounding mode, defaults to Unnecessary.
     *
     * @throws NegativeNumberException    If this number is negative.
     * @throws RoundingNecessaryException If RoundingMode::Unnecessary is used, and the number is not a perfect square.
     *
     * @pure
     */
    public function sqrt(RoundingMode $roundingMode = RoundingMode::Unnecessary): BigInteger
    {}

    /**
     * Returns the integer nth root of this number, rounded according to the given rounding mode.
     *
     * For odd $n, the operation is defined for negative inputs: the sign is preserved and the
     * magnitude of the root is |$this|^(1/$n).
     *
     * @param positive-int $n            The root degree. Must be a strictly positive integer.
     * @param RoundingMode $roundingMode An optional rounding mode, defaults to Unnecessary.
     *
     * @throws InvalidArgumentException   If $n is less than 1.
     * @throws NegativeNumberException    If this number is negative and $n is even.
     * @throws RoundingNecessaryException If RoundingMode::Unnecessary is used, and this number is not a perfect nth power.
     *
     * @pure
     */
    public function nthRoot(int $n, RoundingMode $roundingMode = RoundingMode::Unnecessary): BigInteger
    {}

    #[Override]
    public function negated(): static
    {}

    /**
     * Returns the integer bitwise-and combined with another integer.
     *
     * This method returns a negative BigInteger if and only if both operands are negative.
     *
     * @param BigNumber|int|string $that The operand. Must be convertible to a BigInteger.
     *
     * @throws MathException If the operand is not valid, or is not convertible to a BigInteger.
     *
     * @pure
     */
    public function and(BigNumber|int|string $that): BigInteger
    {}

    /**
     * Returns the integer bitwise-or combined with another integer.
     *
     * This method returns a negative BigInteger if and only if either of the operands is negative.
     *
     * @param BigNumber|int|string $that The operand. Must be convertible to a BigInteger.
     *
     * @throws MathException If the operand is not valid, or is not convertible to a BigInteger.
     *
     * @pure
     */
    public function or(BigNumber|int|string $that): BigInteger
    {}

    /**
     * Returns the integer bitwise-xor combined with another integer.
     *
     * This method returns a negative BigInteger if and only if exactly one of the operands is negative.
     *
     * @param BigNumber|int|string $that The operand. Must be convertible to a BigInteger.
     *
     * @throws MathException If the operand is not valid, or is not convertible to a BigInteger.
     *
     * @pure
     */
    public function xor(BigNumber|int|string $that): BigInteger
    {}

    /**
     * Returns the bitwise-not of this BigInteger.
     *
     * @pure
     */
    public function not(): BigInteger
    {}

    /**
     * Returns the integer left shifted by a given number of bits.
     *
     * If $bits is negative, the integer is shifted right by the absolute value instead.
     *
     * @pure
     */
    public function shiftedLeft(int $bits): BigInteger
    {}

    /**
     * Returns the integer right shifted by a given number of bits.
     *
     * If $bits is negative, the integer is shifted left by the absolute value instead.
     *
     * @pure
     */
    public function shiftedRight(int $bits): BigInteger
    {}

    /**
     * Returns the number of bits in the minimal two's-complement representation of this BigInteger, excluding a sign bit.
     *
     * For positive BigIntegers, this is equivalent to the number of bits in the ordinary binary representation.
     * Computes (ceil(log2(this < 0 ? -this : this+1))).
     *
     * @return non-negative-int
     *
     * @pure
     */
    public function getBitLength(): int
    {}

    /**
     * Returns the index of the rightmost (lowest-order) one bit in this BigInteger.
     *
     * Returns null if this BigInteger is zero.
     *
     * @return non-negative-int|null
     *
     * @pure
     */
    public function getLowestSetBit(): ?int
    {}

    /**
     * Returns true if and only if the designated bit is set.
     *
     * Computes ((this & (1<<bitIndex)) != 0).
     *
     * @param non-negative-int $bitIndex The bit to test, 0-based.
     *
     * @throws InvalidArgumentException If the bit to test is negative.
     *
     * @pure
     */
    public function isBitSet(int $bitIndex): bool
    {}

    /**
     * Returns whether this number is even.
     *
     * @pure
     */
    public function isEven(): bool
    {}

    /**
     * Returns whether this number is odd.
     *
     * @pure
     */
    public function isOdd(): bool
    {}

    #[Override]
    public function compareTo(BigNumber|int|string $that): int
    {}

    #[Override]
    public function getSign(): int
    {}

    #[Override]
    public function toBigInteger(): BigInteger
    {}

    #[Override]
    public function toBigDecimal(): BigDecimal
    {}

    #[Override]
    public function toBigRational(): BigRational
    {}

    #[Override]
    public function toScale(int $scale, RoundingMode $roundingMode = RoundingMode::Unnecessary): BigDecimal
    {}

    #[Override]
    public function toInt(): int
    {}

    #[Override]
    public function toFloat(): float
    {}

    /**
     * Returns a string representation of this number in the given base.
     *
     * The output will always be lowercase for bases greater than 10.
     *
     * @param int<2, 36> $base
     *
     * @return non-empty-string
     *
     * @throws InvalidArgumentException If the base is out of range.
     *
     * @pure
     */
    public function toBase(int $base): string
    {}

    /**
     * Returns a string representation of this number in an arbitrary base with a custom alphabet.
     *
     * This method is byte-oriented: the alphabet is interpreted as a sequence of single-byte characters.
     * Multibyte UTF-8 characters are not supported.
     *
     * Because this method accepts any single-byte character, including dash, it does not handle negative numbers;
     * a NegativeNumberException will be thrown when attempting to call this method on a negative number.
     *
     * @param non-empty-string $alphabet The alphabet, for example '01' for base 2, or '01234567' for base 8.
     *
     * @return non-empty-string
     *
     * @throws InvalidArgumentException If the alphabet does not contain at least 2 chars, or contains duplicates.
     * @throws NegativeNumberException  If this number is negative.
     *
     * @pure
     */
    public function toArbitraryBase(string $alphabet): string
    {}

    /**
     * Returns a string of bytes containing the binary representation of this BigInteger.
     *
     * The string is in big-endian byte-order: the most significant byte is in the zeroth element.
     *
     * If `$signed` is true, the output will be in two's-complement representation, and a sign bit will be prepended to
     * the output. If `$signed` is false, no sign bit will be prepended, and this method will throw an exception if the
     * number is negative.
     *
     * The string will contain the minimum number of bytes required to represent this BigInteger, including a sign bit
     * if `$signed` is true.
     *
     * Note that in signed mode, a positive number whose most significant byte is 0x80 or greater gains an extra leading
     * 0x00 byte, so that its first bit is not interpreted as a sign bit: for example, 128 converts to "\x00\x80" when
     * signed, and to "\x80" when unsigned. Callers that require fixed-width unsigned output, such as cryptographic
     * code, should use `$signed = false`.
     *
     * This representation is compatible with the `fromBytes()` factory method, as long as the `$signed` flags match.
     *
     * @param bool $signed Whether to output a signed number in two's-complement representation with a leading sign bit.
     *
     * @return non-empty-string
     *
     * @throws NegativeNumberException If $signed is false, and the number is negative.
     *
     * @pure
     */
    public function toBytes(bool $signed = true): string
    {}

    /**
     * @return numeric-string
     */
    #[Override]
    public function toString(): string
    {}

    /**
     * This method is required for serializing the object and SHOULD NOT be accessed directly.
     *
     * @internal
     *
     * @return array{value: string}
     */
    public function __serialize(): array
    {}

    /**
     * This method is only here to allow unserializing the object and cannot be accessed directly.
     *
     * @internal
     *
     * @param array{value: string} $data
     *
     * @throws LogicException
     */
    public function __unserialize(array $data): void
    {}

    #[Override]
    protected static function from(BigNumber $number): static
    {}

    #[Override]
    protected function digitCount(): int
    {}

    /**
     * Returns random bytes from the provided generator or from random_bytes().
     *
     * @param int                          $byteLength           The number of requested bytes.
     * @param (callable(int): string)|null $randomBytesGenerator The random bytes generator, or null to use random_bytes().
     *
     * @throws RandomSourceException If random byte generation fails.
     */
    private static function randomBytes(int $byteLength, ?callable $randomBytesGenerator): string
    {}

    /**
     * @pure
     */
    private function isOne(): bool
    {}

    /**
     * @pure
     */
    private function isMinusOne(): bool
    {}
}
