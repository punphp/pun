<?php

declare(strict_types=1);

namespace Brick\Math\Internal\Calculator;

use Brick\Math\Exception\PlatformException;
use Brick\Math\Internal\Calculator;
use Override;

use function assert;
use function in_array;
use function intdiv;
use function is_int;
use function ltrim;
use function str_pad;
use function str_repeat;
use function strcmp;
use function strlen;
use function substr;

use const PHP_INT_SIZE;
use const STR_PAD_LEFT;

/**
 * Calculator implementation using only native PHP code.
 *
 * @internal
 */
final readonly class NativeCalculator extends Calculator
{
    /**
     * The max number of digits the platform can natively add, subtract, multiply or divide without overflow.
     * For multiplication, this represents the max sum of the lengths of both operands.
     *
     * In addition, it is assumed that an extra digit can hold a carry (1) without overflowing.
     * Example: 32-bit: max number 1,999,999,999 (9 digits + carry)
     *          64-bit: max number 1,999,999,999,999,999,999 (18 digits + carry)
     */
    private int $maxDigits;

    /**
     * @pure
     *
     * @codeCoverageIgnore
     */
    public function __construct()
    {}

    #[Override]
    public function add(string $a, string $b): string
    {}

    #[Override]
    public function sub(string $a, string $b): string
    {}

    #[Override]
    public function mul(string $a, string $b): string
    {}

    #[Override]
    public function divQ(string $a, string $b): string
    {}

    #[Override]
    public function divR(string $a, string $b): string
    {}

    #[Override]
    public function divQR(string $a, string $b): array
    {}

    #[Override]
    public function pow(string $a, int $e): string
    {}

    /**
     * Algorithm from: https://www.geeksforgeeks.org/modular-exponentiation-power-in-modular-arithmetic/.
     */
    #[Override]
    public function modPow(string $base, string $exp, string $mod): string
    {}

    /**
     * Adapted from https://cp-algorithms.com/num_methods/roots_newton.html.
     */
    #[Override]
    public function sqrt(string $n): string
    {}

    /**
     * Performs the addition of two non-signed large integers.
     *
     * @pure
     */
    private function doAdd(string $a, string $b): string
    {}

    /**
     * Performs the subtraction of two non-signed large integers.
     *
     * @pure
     */
    private function doSub(string $a, string $b): string
    {}

    /**
     * Performs the multiplication of two non-signed large integers.
     *
     * @pure
     */
    private function doMul(string $a, string $b): string
    {}

    /**
     * Performs the division of two non-signed large integers.
     *
     * @return string[] The quotient and remainder.
     *
     * @pure
     */
    private function doDiv(string $a, string $b): array
    {}

    /**
     * Compares two non-signed large numbers.
     *
     * @return -1|0|1
     *
     * @pure
     */
    private function doCmp(string $a, string $b): int
    {}

    /**
     * Pads the left of one of the given numbers with zeros if necessary to make both numbers the same length.
     *
     * The numbers must only consist of digits, without leading minus sign.
     *
     * @return array{string, string, int}
     *
     * @pure
     */
    private function pad(string $a, string $b): array
    {}
}
