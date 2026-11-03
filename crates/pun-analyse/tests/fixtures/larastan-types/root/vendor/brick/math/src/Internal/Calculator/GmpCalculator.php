<?php

declare(strict_types=1);

namespace Brick\Math\Internal\Calculator;

use Brick\Math\Internal\Calculator;
use GMP;
use Override;

use function gmp_add;
use function gmp_and;
use function gmp_div_q;
use function gmp_div_qr;
use function gmp_div_r;
use function gmp_gcd;
use function gmp_init;
use function gmp_invert;
use function gmp_lcm;
use function gmp_mul;
use function gmp_or;
use function gmp_pow;
use function gmp_powm;
use function gmp_root;
use function gmp_sqrt;
use function gmp_strval;
use function gmp_sub;
use function gmp_xor;
use function substr;

/**
 * Calculator implementation built around the GMP library.
 *
 * @internal
 */
final readonly class GmpCalculator extends Calculator
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
    public function modInverse(string $x, string $m): ?string
    {}

    #[Override]
    public function modPow(string $base, string $exp, string $mod): string
    {}

    #[Override]
    public function gcd(string $a, string $b): string
    {}

    #[Override]
    public function lcm(string $a, string $b): string
    {}

    #[Override]
    public function fromBase(string $number, int $base): string
    {}

    #[Override]
    public function toBase(string $number, int $base): string
    {}

    #[Override]
    public function and(string $a, string $b): string
    {}

    #[Override]
    public function or(string $a, string $b): string
    {}

    #[Override]
    public function xor(string $a, string $b): string
    {}

    #[Override]
    public function sqrt(string $n): string
    {}

    #[Override]
    public function nthRoot(string $n, int $k): string
    {}
}
