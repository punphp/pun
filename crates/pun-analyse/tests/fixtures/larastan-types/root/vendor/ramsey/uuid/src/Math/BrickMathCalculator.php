<?php

/**
 * This file is part of the ramsey/uuid library
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 *
 * @copyright Copyright (c) Ben Ramsey <ben@benramsey.com>
 * @license http://opensource.org/licenses/MIT MIT
 */

declare(strict_types=1);

namespace Ramsey\Uuid\Math;

use Brick\Math\BigDecimal;
use Brick\Math\BigInteger;
use Brick\Math\Exception\MathException;
use Ramsey\Uuid\Exception\InvalidArgumentException;
use Ramsey\Uuid\Type\Decimal;
use Ramsey\Uuid\Type\Hexadecimal;
use Ramsey\Uuid\Type\Integer as IntegerObject;
use Ramsey\Uuid\Type\NumberInterface;

/**
 * A calculator using the brick/math library for arbitrary-precision arithmetic
 *
 * @immutable
 */
final class BrickMathCalculator implements CalculatorInterface
{
    public function add(NumberInterface $augend, NumberInterface ...$addends): NumberInterface
    {}

    public function subtract(NumberInterface $minuend, NumberInterface ...$subtrahends): NumberInterface
    {}

    public function multiply(NumberInterface $multiplicand, NumberInterface ...$multipliers): NumberInterface
    {}

    public function divide(
        int $roundingMode,
        int $scale,
        NumberInterface $dividend,
        NumberInterface ...$divisors,
    ): NumberInterface {}

    public function fromBase(string $value, int $base): IntegerObject
    {}

    public function toBase(IntegerObject $value, int $base): string
    {}

    public function toHexadecimal(IntegerObject $value): Hexadecimal
    {}

    public function toInteger(Hexadecimal $value): IntegerObject
    {}
}
