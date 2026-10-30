<?php

namespace WideIntegerKeys;

function decimalKeys(): void
{
    $duplicate = [18446744073709555712 => 0, 4096 => 1];
    $distinct = [18446744073709555712 => 0, 0 => 1];
    $reversed = [4096 => 0, 18_446_744_073_709_555_712 => 1];
    $negative = [-18446744073709555712 => 0, -4096 => 1];
    $negativeDistinct = [-18446744073709555712 => 0, 0 => 1];
    $key = 18446744073709555712;
    $copy = $key;
    \PHPStan\dumpType($copy);
    $copied = [$copy => 0, 4096 => 1];
    $copiedDistinct = [$copy => 0, 0 => 1];
    $copy = -18446744073709555712;
    \PHPStan\dumpType($copy);
    $copiedNegative = [$copy => 0, -4096 => 1];
}

function radixKeys(): void
{
    $hex = [0x10000000000001000 => 0, 4096 => 1];
    $hexDistinct = [0x10000000000001000 => 0, 0 => 1];
    $binary = [0b10000000000000000000000000000000000000000000000000001000000000000 => 0, 4096 => 1];
    $octal = [02000000000000000010000 => 0, 4096 => 1];
    $explicitOctal = [0o2000000000000000010000 => 0, 4096 => 1];
    $separated = [0X1_0000_0000_0000_1000 => 0, 4096 => 1];
    $separatedBinary = [0B1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000_0000_0000 => 0, 4096 => 1];
    $separatedOctal = [0O2_000_000_000_000_000_010_000 => 0, 4096 => 1];
    $negative = [-0x10000000000001000 => 0, -4096 => 1];
    $key = 0x10000000000001000;
    \PHPStan\dumpType($key);
    $copied = [$key => 0, 4096 => 1];
    $key = 0b10000000000000000000000000000000000000000000000000001000000000000;
    $copiedBinary = [$key => 0, 4096 => 1];
    $key = 0o2000000000000000010000;
    $copiedOctal = [$key => 0, 4096 => 1];
}

function roundingAndSignedBoundaries(): void
{
    $max = [18446744073709551615 => 0, 0 => 1];
    $next = [18446744073709551616 => 0, 0 => 1];
    $below = [18446744073709549568 => 0, -2048 => 1];
    $tie = [18446744073709553664 => 0, 0 => 1];
    $aboveTie = [18446744073709553665 => 0, 4096 => 1];
    $hexTie = [0x10000000000000800 => 0, 0 => 1];
    $hexAboveTie = [0x10000000000000801 => 0, 0 => 1];
    $hexRoundUp = [0x10000000000000fff => 0, 4096 => 1];
    $intMax = [9223372036854775807 => 0, 0x7fffffffffffffff => 1];
    $intMin = [-9223372036854775808 => 0, -9223372036854775807 - 1 => 1];
    $nearMin = [-9223372036854775807 => 0, -9223372036854775808 => 1];
    $key = -9223372036854775808;
    \PHPStan\dumpType($key);
    $copiedMin = [$key => 0, -9223372036854775807 - 1 => 1];
    $zero = 0; $hexZero = 0x0; $octalZero = 0o0; $binaryZero = 0b0;
}

const WIDE = 18446744073709555712;
const HUGE = 0x100000000000000000000000000000001;

class Constants
{
    public const WIDE = 0x10000000000001000;

    public function keys(): void
    {
        $global = [WIDE => 0, 4096 => 1];
        $member = [self::WIDE => 0, 4096 => 1];
        $huge = [HUGE => 0, 0 => 1];
    }
}

function largeMagnitudes(): void
{
    $decimal = 340282366920938463463374607431768211456;
    $hex = 0x100000000000000000000000000000000;
    $upperHex = 0XFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;
    $binary = 0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000;
    $octal = 020000000000000000000000000000000000000000000000;
    \PHPStan\dumpType($decimal);
    \PHPStan\dumpType($hex);
    \PHPStan\dumpType($binary);
    \PHPStan\dumpType($octal);
    $keys = [$decimal => 0, $hex => 1, $binary => 2, $octal => 3, 0 => 4];
    $negative = [-0x100000000000000000000000000000000 => 0, 0 => 1];
    $infinite = 9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999;
    \PHPStan\dumpType($infinite);
    $infiniteKey = [$infinite => 0, 0 => 1];
    $hexInfinite = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
    \PHPStan\dumpType($hexInfinite);
    $hexInfiniteKey = [$hexInfinite => 0, 0 => 1];
    $negativeInfiniteKey = [-$infinite => 0, 0 => 1];
}
