<?php

declare(strict_types=1);

namespace Brick\Math\Internal\Calculator;

use Brick\Math\Internal\Calculator;
use Override;

use function bcadd;
use function bcdiv;
use function bcmod;
use function bcmul;
use function bcpow;
use function bcpowmod;
use function bcsqrt;
use function bcsub;

/**
 * Calculator implementation built around the bcmath library.
 *
 * @internal
 */
final readonly class BcMathCalculator extends Calculator
{
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

    #[Override]
    public function modPow(string $base, string $exp, string $mod): string
    {}

    #[Override]
    public function sqrt(string $n): string
    {}
}
