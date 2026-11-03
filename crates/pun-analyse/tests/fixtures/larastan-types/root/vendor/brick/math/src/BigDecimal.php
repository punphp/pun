<?php

declare(strict_types=1);

namespace Brick\Math;

use Brick\Math\Exception\DivisionByZeroException;
use Brick\Math\Exception\InvalidArgumentException;
use Brick\Math\Exception\MathException;
use Brick\Math\Exception\NegativeNumberException;
use Brick\Math\Exception\PlatformException;
use Brick\Math\Exception\RoundingNecessaryException;
use Brick\Math\Internal\CalculatorRegistry;
use Brick\Math\Internal\DecimalHelper;
use Brick\Math\Internal\Safe;
use LogicException;
use Override;

use function assert;
use function chr;
use function in_array;
use function ini_set;
use function intdiv;
use function is_infinite;
use function is_nan;
use function json_encode;
use function max;
use function pack;
use function rtrim;
use function str_repeat;
use function strlen;
use function substr;
use function unpack;

use const PHP_INT_SIZE;

/**
 * An arbitrarily large decimal number.
 *
 * This class is immutable.
 *
 * The scale of the number is the number of digits after the decimal point. It is always positive or zero.
 */
final readonly class BigDecimal extends BigNumber
{
    /**
     * The unscaled value of this decimal number.
     *
     * This is a string of digits with an optional leading minus sign.
     * No leading zero must be present.
     * No leading minus sign must be present if the value is 0.
     */
    private string $value;

    /**
     * The scale (number of digits after the decimal point) of this decimal number.
     *
     * This must be zero or more.
     *
     * @var non-negative-int
     */
    private int $scale;

    /**
     * Protected constructor. Use a factory method to obtain an instance.
     *
     * @param string           $value The unscaled value, validated.
     * @param non-negative-int $scale The scale, validated.
     *
     * @pure
     */
    protected function __construct(string $value, int $scale = 0)
    {}

    /**
     * Creates a BigDecimal from an unscaled value and a scale.
     *
     * Example: `(12345, 3)` will result in the BigDecimal `12.345`.
     *
     * A negative scale is normalized to zero by appending zeros to the unscaled value.
     *
     * Example: `(12345, -3)` will result in the BigDecimal `12345000`.
     *
     * @param BigNumber|int|string $value The unscaled value. Must be convertible to a BigInteger.
     * @param int                  $scale The scale of the number. If negative, the scale will be set to zero
     *                                    and the unscaled value will be adjusted accordingly.
     *
     * @throws MathException If the value is not valid, or is not convertible to a BigInteger.
     *
     * @pure
     */
    public static function ofUnscaledValue(BigNumber|int|string $value, int $scale = 0): BigDecimal
    {}

    /**
     * Returns a BigDecimal representing zero, with a scale of zero.
     *
     * @pure
     */
    public static function zero(): BigDecimal
    {}

    /**
     * Returns a BigDecimal representing one, with a scale of zero.
     *
     * @pure
     */
    public static function one(): BigDecimal
    {}

    /**
     * Returns a BigDecimal representing ten, with a scale of zero.
     *
     * @pure
     */
    public static function ten(): BigDecimal
    {}

    /**
     * Creates a BigDecimal from the exact IEEE-754 value of a float.
     *
     * Examples:
     *   - `fromFloatExact(0.1)` returns a BigDecimal with value '0.1000000000000000055511151231257827021181583404541015625'
     *   - `fromFloatExact(0.3)` returns a BigDecimal with value '0.299999999999999988897769753748434595763683319091796875'
     *   - `fromFloatExact(0.5)` returns a BigDecimal with value '0.5'
     *   - `fromFloatExact(1.0)` returns a BigDecimal with value '1'
     *
     * Note that BigDecimal has no concept of negative zero, so `-0.0` and `0.0` both convert to zero.
     *
     * @throws InvalidArgumentException If the value is NaN or infinite.
     * @throws PlatformException        If the platform uses a non-IEEE-754 double format.
     *
     * @pure
     */
    public static function fromFloatExact(float $value): BigDecimal
    {}

    /**
     * Creates a BigDecimal from the shortest decimal representation of a float that round-trips back to the same value.
     *
     * The result is the shortest BigDecimal that passes `BigDecimal::fromFloatShortest($f)->toFloat() === $f`.
     *
     * Examples:
     *   - `fromFloatShortest(0.3)` returns a BigDecimal with value '0.3'
     *   - `fromFloatShortest(0.1 * 3.0)` returns a BigDecimal with value '0.30000000000000004' (`0.1 * 3.0 !== 0.3`)
     *   - `fromFloatShortest(1.0 / 3.0)` returns a BigDecimal with value '0.3333333333333333'
     *
     * Note that BigDecimal has no concept of negative zero, so `-0.0` and `0.0` both convert to zero.
     *
     * @throws InvalidArgumentException If the value is NaN or infinite.
     */
    public static function fromFloatShortest(float $value): BigDecimal
    {}

    /**
     * Returns the sum of this number and the given one.
     *
     * The result has a scale of `max($this->scale, $that->scale)`.
     *
     * @param BigNumber|int|string $that The number to add. Must be convertible to a BigDecimal.
     *
     * @throws MathException If the number is not valid, or is not convertible to a BigDecimal.
     *
     * @pure
     */
    public function plus(BigNumber|int|string $that): BigDecimal
    {}

    /**
     * Returns the difference of this number and the given one.
     *
     * The result has a scale of `max($this->scale, $that->scale)`.
     *
     * @param BigNumber|int|string $that The number to subtract. Must be convertible to a BigDecimal.
     *
     * @throws MathException If the number is not valid, or is not convertible to a BigDecimal.
     *
     * @pure
     */
    public function minus(BigNumber|int|string $that): BigDecimal
    {}

    /**
     * Returns the product of this number and the given one.
     *
     * The result has a scale of `$this->scale + $that->scale`.
     *
     * @param BigNumber|int|string $that The multiplier. Must be convertible to a BigDecimal.
     *
     * @throws MathException If the multiplier is not valid, or is not convertible to a BigDecimal.
     *
     * @pure
     */
    public function multipliedBy(BigNumber|int|string $that): BigDecimal
    {}

    /**
     * Returns the result of the division of this number by the given one, at the given scale.
     *
     * @param BigNumber|int|string $that         The divisor. Must be convertible to a BigDecimal.
     * @param non-negative-int     $scale        The desired scale. Must be non-negative.
     * @param RoundingMode         $roundingMode An optional rounding mode, defaults to Unnecessary.
     *
     * @throws MathException              If the divisor is not valid, or is not convertible to a BigDecimal.
     * @throws InvalidArgumentException   If the scale is negative.
     * @throws DivisionByZeroException    If the divisor is zero.
     * @throws RoundingNecessaryException If RoundingMode::Unnecessary is used and the result cannot be represented
     *                                    exactly at the given scale.
     *
     * @pure
     */
    public function dividedBy(BigNumber|int|string $that, int $scale, RoundingMode $roundingMode = RoundingMode::Unnecessary): BigDecimal
    {}

    /**
     * Returns the exact result of the division of this number by the given one.
     *
     * The scale of the result is automatically calculated to fit all the fraction digits.
     *
     * @param BigNumber|int|string $that The divisor. Must be convertible to a BigDecimal.
     *
     * @throws MathException              If the divisor is not valid, or is not convertible to a BigDecimal.
     * @throws DivisionByZeroException    If the divisor is zero.
     * @throws RoundingNecessaryException If the result yields an infinite number of digits.
     *
     * @pure
     */
    public function dividedByExact(BigNumber|int|string $that): BigDecimal
    {}

    /**
     * Returns this number exponentiated to the given value.
     *
     * The result has a scale of `$this->scale * $exponent`.
     *
     * @param non-negative-int $exponent
     *
     * @throws InvalidArgumentException If the exponent is negative.
     *
     * @pure
     */
    public function power(int $exponent): BigDecimal
    {}

    /**
     * Returns the quotient of the division of this number by the given one.
     *
     * The quotient has a scale of `0`.
     *
     * Examples:
     *
     * - `7.5` quotient `3` returns `2`
     * - `7.5` quotient `-3` returns `-2`
     * - `-7.5` quotient `3` returns `-2`
     * - `-7.5` quotient `-3` returns `2`
     *
     * @param BigNumber|int|string $that The divisor. Must be convertible to a BigDecimal.
     *
     * @throws MathException           If the divisor is not valid, or is not convertible to a BigDecimal.
     * @throws DivisionByZeroException If the divisor is zero.
     *
     * @pure
     */
    public function quotient(BigNumber|int|string $that): BigDecimal
    {}

    /**
     * Returns the remainder of the division of this number by the given one.
     *
     * The remainder has a scale of `max($this->scale, $that->scale)`.
     * The remainder, when non-zero, has the same sign as the dividend.
     *
     * Examples:
     *
     * - `7.5` remainder `3` returns `1.5`
     * - `7.5` remainder `-3` returns `1.5`
     * - `-7.5` remainder `3` returns `-1.5`
     * - `-7.5` remainder `-3` returns `-1.5`
     *
     * @param BigNumber|int|string $that The divisor. Must be convertible to a BigDecimal.
     *
     * @throws MathException           If the divisor is not valid, or is not convertible to a BigDecimal.
     * @throws DivisionByZeroException If the divisor is zero.
     *
     * @pure
     */
    public function remainder(BigNumber|int|string $that): BigDecimal
    {}

    /**
     * Returns the quotient and remainder of the division of this number by the given one.
     *
     * The quotient has a scale of `0`, and the remainder has a scale of `max($this->scale, $that->scale)`.
     *
     * Examples:
     *
     * - `7.5` quotientAndRemainder `3` returns [`2`, `1.5`]
     * - `7.5` quotientAndRemainder `-3` returns [`-2`, `1.5`]
     * - `-7.5` quotientAndRemainder `3` returns [`-2`, `-1.5`]
     * - `-7.5` quotientAndRemainder `-3` returns [`2`, `-1.5`]
     *
     * @param BigNumber|int|string $that The divisor. Must be convertible to a BigDecimal.
     *
     * @return array{BigDecimal, BigDecimal} An array containing the quotient and the remainder.
     *
     * @throws MathException           If the divisor is not valid, or is not convertible to a BigDecimal.
     * @throws DivisionByZeroException If the divisor is zero.
     *
     * @pure
     */
    public function quotientAndRemainder(BigNumber|int|string $that): array
    {}

    /**
     * Returns the square root of this number, rounded to the given scale according to the given rounding mode.
     *
     * @param non-negative-int $scale        The target scale. Must be non-negative.
     * @param RoundingMode     $roundingMode An optional rounding mode, defaults to Unnecessary.
     *
     * @throws InvalidArgumentException   If the scale is negative.
     * @throws NegativeNumberException    If this number is negative.
     * @throws RoundingNecessaryException If RoundingMode::Unnecessary is used and the result cannot be represented
     *                                    exactly at the given scale.
     *
     * @pure
     */
    public function sqrt(int $scale, RoundingMode $roundingMode = RoundingMode::Unnecessary): BigDecimal
    {}

    /**
     * Returns the nth root of this number, rounded to the given scale according to the given rounding mode.
     *
     * For odd $n, the operation is defined for negative inputs: the sign is preserved and the
     * magnitude of the root is |$this|^(1/$n).
     *
     * @param positive-int     $n            The root degree. Must be a strictly positive integer.
     * @param non-negative-int $scale        The target scale. Must be non-negative.
     * @param RoundingMode     $roundingMode An optional rounding mode, defaults to Unnecessary.
     *
     * @throws InvalidArgumentException   If $n is less than 1 or $scale is negative.
     * @throws NegativeNumberException    If this number is negative and $n is even.
     * @throws RoundingNecessaryException If RoundingMode::Unnecessary is used and the result cannot be represented
     *                                    exactly at the given scale.
     *
     * @pure
     */
    public function nthRoot(int $n, int $scale, RoundingMode $roundingMode = RoundingMode::Unnecessary): BigDecimal
    {}

    /**
     * Returns a copy of this BigDecimal with the decimal point moved to the left by the given number of places.
     *
     * If $places is negative, the decimal point is moved to the right by the absolute value instead.
     *
     * @pure
     */
    public function withPointMovedLeft(int $places): BigDecimal
    {}

    /**
     * Returns a copy of this BigDecimal with the decimal point moved to the right by the given number of places.
     *
     * If $places is negative, the decimal point is moved to the left by the absolute value instead.
     *
     * @pure
     */
    public function withPointMovedRight(int $places): BigDecimal
    {}

    /**
     * Returns a copy of this BigDecimal with any trailing zeros removed from the fractional part.
     *
     * Examples:
     *
     * - `1.200` returns `1.2`
     * - `1.000` returns `1`
     * - `100` returns `100`
     *
     * @pure
     */
    public function strippedOfTrailingZeros(): BigDecimal
    {}

    #[Override]
    public function negated(): static
    {}

    #[Override]
    public function compareTo(BigNumber|int|string $that): int
    {}

    #[Override]
    public function getSign(): int
    {}

    /**
     * Returns the unscaled value of this decimal number.
     *
     * For example, the unscaled value of `123.456` is `123456`.
     *
     * @pure
     */
    public function getUnscaledValue(): BigInteger
    {}

    /**
     * Returns the scale of this decimal number.
     *
     * The scale is the number of digits after the decimal point. For example, the scale of `123.456` is `3`.
     *
     * @return non-negative-int
     *
     * @pure
     */
    public function getScale(): int
    {}

    /**
     * Returns the number of significant digits in the number.
     *
     * This is the number of digits in the unscaled value of the number.
     * The sign has no impact on the result.
     *
     * Examples:
     *   0 => 1
     *   0.0 => 1
     *   123 => 3
     *   123.456 => 6
     *   0.00123 => 3
     *   0.0012300 => 5
     *
     * @return positive-int
     *
     * @pure
     */
    public function getPrecision(): int
    {}

    /**
     * Returns the integral part of this decimal number.
     *
     * Examples:
     *
     * - `123.456` returns `123`
     * - `-123.456` returns `-123`
     * - `0.123` returns `0`
     * - `-0.123` returns `0`
     *
     * The following identity holds: `$d->isEqualTo($d->getFractionalPart()->plus($d->getIntegralPart()))`. Note that in
     * this identity, the operand order is significant: the reversed form throws when the fractional part is non-zero.
     *
     * @pure
     */
    public function getIntegralPart(): BigInteger
    {}

    /**
     * Returns the fractional part of this decimal number.
     *
     * Examples:
     *
     * - `123.456` returns `0.456`
     * - `-123.456` returns `-0.456`
     * - `123` returns `0`
     * - `-123` returns `0`
     * - `123.000` returns `0.000`
     *
     * The result always has the same scale as `$this`.
     *
     * The following identity holds: `$d->isEqualTo($d->getFractionalPart()->plus($d->getIntegralPart()))`. Note that in
     * this identity, the operand order is significant: the reversed form throws when the fractional part is non-zero.
     *
     * @pure
     */
    public function getFractionalPart(): BigDecimal
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
     * @return array{value: string, scale: non-negative-int}
     */
    public function __serialize(): array
    {}

    /**
     * This method is only here to allow unserializing the object and cannot be accessed directly.
     *
     * @internal
     *
     * @param array{value: string, scale: non-negative-int} $data
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
     * Puts the internal values of the given decimal numbers on the same scale.
     *
     * @return array{string, string} The scaled integer values of $x and $y.
     *
     * @pure
     */
    private function scaleValues(BigDecimal $x, BigDecimal $y): array
    {}

    /**
     * @pure
     */
    private function valueWithMinScale(int $scale): string
    {}

    /**
     * @pure
     */
    private function isOneScaleZero(): bool
    {}
}
