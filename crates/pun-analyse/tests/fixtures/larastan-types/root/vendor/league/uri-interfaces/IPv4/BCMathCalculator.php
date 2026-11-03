<?php

/**
 * League.Uri (https://uri.thephpleague.com)
 *
 * (c) Ignace Nyamagana Butera <nyamsprod@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

declare(strict_types=1);

namespace League\Uri\IPv4;

use function bcadd;
use function bccomp;
use function bcdiv;
use function bcmod;
use function bcmul;
use function bcpow;
use function bcsub;
use function str_split;

final class BCMathCalculator implements Calculator
{
    private const SCALE = 0;
    private const CONVERSION_TABLE = [
        '0' => '0', '1' => '1', '2' => '2', '3' => '3',
        '4' => '4', '5' => '5', '6' => '6', '7' => '7',
        '8' => '8', '9' => '9', 'a' => '10', 'b' => '11',
        'c' => '12', 'd' => '13', 'e' => '14', 'f' => '15',
    ];

    public function baseConvert(mixed $value, int $base): string
    {}

    public function pow(mixed $value, int $exponent): string
    {}

    public function compare(mixed $value1, mixed $value2): int
    {}

    public function multiply(mixed $value1, mixed $value2): string
    {}

    public function div(mixed $value, mixed $base): string
    {}

    public function mod(mixed $value, mixed $base): string
    {}

    public function add(mixed $value1, mixed $value2): string
    {}

    public function sub(mixed $value1, mixed $value2): string
    {}
}
