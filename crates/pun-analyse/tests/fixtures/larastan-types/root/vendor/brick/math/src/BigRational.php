<?php

declare(strict_types=1);

namespace Brick\Math;

use Brick\Math\Exception\DivisionByZeroException;
use Brick\Math\Exception\InvalidArgumentException;
use Brick\Math\Exception\MathException;
use Brick\Math\Exception\RoundingNecessaryException;
use Brick\Math\Internal\DecimalHelper;
use Brick\Math\Internal\Safe;
use LogicException;
use Override;

use function max;
use function min;
use function strlen;
use function substr;

/**
 * An arbitrarily large rational number.
 *
 * This class is immutable.
 *
 * Fractions are automatically simplified to lowest terms. For example, `2/4` becomes `1/2`.
 * The denominator is always strictly positive; the sign is carried by the numerator.
 */
final readonly class BigRational extends BigNumber
{
    /**
     * The numerator.
     */
    private BigInteger $numerator;

    /**
     * The denominator. Always strictly positive.
     */
    private BigInteger $denominator;

    /**
     * Protected constructor. Use a factory method to obtain an instance.
     *
     * @param BigInteger $numerator        The numerator.
     * @param BigInteger $denominator      The denominator.
     * @param bool       $checkDenominator Whether to check the denominator for negative and zero.
     * @param bool       $simplify         Whether to simplify the fraction to lowest terms.
     *
     * @throws DivisionByZeroException If the denominator is zero.
     *
     * @pure
     */
    protected function __construct(BigInteger $numerator, BigInteger $denominator, bool $checkDenominator, bool $simplify)
    {}

    /**
     * Creates a BigRational out of a numerator and a denominator.
     *
     * If the denominator is negative, the signs of both the numerator and the denominator
     * will be inverted to ensure that the denominator is always positive.
     *
     * @param BigNumber|int|string $numerator   The numerator. Must be convertible to a BigInteger.
     * @param BigNumber|int|string $denominator The denominator. Must be convertible to a BigInteger.
     *
     * @throws MathException           If an argument is not valid, or is not convertible to a BigInteger.
     * @throws DivisionByZeroException If the denominator is zero.
     *
     * @pure
     */
    public static function ofFraction(
        BigNumber|int|string $numerator,
        BigNumber|int|string $denominator,
    ): BigRational {}

    /**
     * Returns a BigRational representing zero.
     *
     * @pure
     */
    public static function zero(): BigRational
    {}

    /**
     * Returns a BigRational representing one.
     *
     * @pure
     */
    public static function one(): BigRational
    {}

    /**
     * Returns a BigRational representing ten.
     *
     * @pure
     */
    public static function ten(): BigRational
    {}

    /**
     * Returns the numerator of this rational number.
     *
     * @pure
     */
    public function getNumerator(): BigInteger
    {}

    /**
     * Returns the denominator of this rational number.
     *
     * The denominator is always strictly positive.
     *
     * @pure
     */
    public function getDenominator(): BigInteger
    {}

    /**
     * Returns the integral part of this rational number.
     *
     * Examples:
     *
     * - `7/3` returns `2` (since 7/3 = 2 + 1/3)
     * - `-7/3` returns `-2` (since -7/3 = -2 + (-1/3))
     *
     * The following identity holds: `$r->isEqualTo($r->getFractionalPart()->plus($r->getIntegralPart()))`. Note that in
     * this identity, the operand order is significant: the reversed form throws when the fractional part is non-zero.
     *
     * @pure
     */
    public function getIntegralPart(): BigInteger
    {}

    /**
     * Returns the fractional part of this rational number.
     *
     * Examples:
     *
     * - `7/3` returns `1/3` (since 7/3 = 2 + 1/3)
     * - `-7/3` returns `-1/3` (since -7/3 = -2 + (-1/3))
     *
     * The following identity holds: `$r->isEqualTo($r->getFractionalPart()->plus($r->getIntegralPart()))`. Note that in
     * this identity, the operand order is significant: the reversed form throws when the fractional part is non-zero.
     *
     * @pure
     */
    public function getFractionalPart(): BigRational
    {}

    /**
     * Returns the sum of this number and the given one.
     *
     * @param BigNumber|int|string $that The number to add.
     *
     * @throws MathException If the number is not valid.
     *
     * @pure
     */
    public function plus(BigNumber|int|string $that): BigRational
    {}

    /**
     * Returns the difference of this number and the given one.
     *
     * @param BigNumber|int|string $that The number to subtract.
     *
     * @throws MathException If the number is not valid.
     *
     * @pure
     */
    public function minus(BigNumber|int|string $that): BigRational
    {}

    /**
     * Returns the product of this number and the given one.
     *
     * @param BigNumber|int|string $that The multiplier.
     *
     * @throws MathException If the multiplier is not valid.
     *
     * @pure
     */
    public function multipliedBy(BigNumber|int|string $that): BigRational
    {}

    /**
     * Returns the result of the division of this number by the given one.
     *
     * @param BigNumber|int|string $that The divisor.
     *
     * @throws MathException           If the divisor is not valid.
     * @throws DivisionByZeroException If the divisor is zero.
     *
     * @pure
     */
    public function dividedBy(BigNumber|int|string $that): BigRational
    {}

    /**
     * Returns this number exponentiated to the given value.
     *
     * Unlike BigInteger and BigDecimal, BigRational supports negative exponents:
     * the result is the reciprocal raised to the absolute value of the exponent.
     *
     * @throws DivisionByZeroException If the exponent is negative and this number is zero.
     *
     * @pure
     */
    public function power(int $exponent): BigRational
    {}

    /**
     * Returns the reciprocal of this BigRational.
     *
     * The reciprocal has the numerator and denominator swapped.
     *
     * @throws DivisionByZeroException If this number is zero.
     *
     * @pure
     */
    public function reciprocal(): BigRational
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

    #[Override]
    public function toString(): string
    {}

    /**
     * Returns the decimal representation of this rational number, with repeating decimals in parentheses.
     *
     * WARNING: This method is unbounded.
     *          The length of the repeating decimal period can be as large as `denominator - 1`.
     *          For fractions with large denominators, this method can use excessive memory and CPU time.
     *          For example, `1/100019` has a repeating period of 100,018 digits.
     *
     * Examples:
     *
     * - `10/3` returns `3.(3)`
     * - `171/70` returns `2.4(428571)`
     * - `1/2` returns `0.5`
     *
     * @return non-empty-string
     *
     * @pure
     */
    public function toRepeatingDecimalString(): string
    {}

    /**
     * This method is required for serializing the object and SHOULD NOT be accessed directly.
     *
     * @internal
     *
     * @return array{numerator: BigInteger, denominator: BigInteger}
     */
    public function __serialize(): array
    {}

    /**
     * This method is only here to allow unserializing the object and cannot be accessed directly.
     *
     * @internal
     *
     * @param array{numerator: BigInteger, denominator: BigInteger} $data
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
}
