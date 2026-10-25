<?php

namespace FullConstantDuplicateKeys {
    use const PHP_INT_SIZE as WORD_SIZE;

    const OVERFLOW = 1e308 * 1e308;
    const NONFINITE = \INF - \INF;
    const MODULO = 5.9 % 2.0;
    const QUALIFIED_SIZE = \PHP_INT_SIZE + 1;

    function constants(): void
    {
        $a = [OVERFLOW => 0, 0 => 1];
        $b = [NONFINITE => 0, 0 => 1];
        $c = [MODULO => 0, 1 => 1];
        $d = [5 => 0, 9 => 1, QUALIFIED_SIZE => 2];
    }

    function runtime(): void
    {
        $overflow = 1e308 * 1e308;
        \PHPStan\dumpType($overflow);
        $a = [$overflow => 0, 0 => 1];
        $nonfinite = \INF - \INF;
        \PHPStan\dumpType($nonfinite);
        $b = [$nonfinite => 0, 0 => 1];
        \PHPStan\dumpType(\INF);
        \PHPStan\dumpType(\NAN);
        \PHPStan\dumpType(INF);
        \PHPStan\dumpType(NAN);
        \PHPStan\dumpType(\PHP_INT_SIZE);
        \PHPStan\dumpType(PHP_INT_SIZE);
        \PHPStan\dumpType(WORD_SIZE);
        \PHPStan\dumpType(5.9 % 2.0);
        \PHPStan\dumpType(-5.9 % 2.0);
        \PHPStan\dumpType(5.9 % -2.0);
        \PHPStan\dumpType(5 % 2.9);
        \PHPStan\dumpType(9007199254740993 % 2.0);
        \PHPStan\dumpType(9223372036854775808.0 % 3);
        \PHPStan\dumpType(-32281802128991715328.0 % 3);
        \PHPStan\dumpType(-18446744073709551616.0 % 3);
        \PHPStan\dumpType(1e50 % 3);
        \PHPStan\dumpType(-1e50 % 3);
        \PHPStan\dumpType(\INF % 2);
        \PHPStan\dumpType(\NAN % 2);
        \PHPStan\dumpType(2.0 % 3);
        $min = -9223372036854775807 - 1;
        \PHPStan\dumpType($min % -1.0);
        \PHPStan\dumpType($min / -1);
    }

    function copies(): void
    {
        $key = MODULO;
        \PHPStan\dumpType($key);
        $a = [$key => 0, 1 => 1];
        $key = 7;
        $b = [$key => 0, 1 => 1];
        $size = WORD_SIZE;
        assert($size === 4);
        \PHPStan\dumpType($size);
        $c = [4 => 0, 8 => 1, $size => 2];
    }

    function unknown(float $left, float $right): void
    {
        $product = $left * $right;
        \PHPStan\dumpType($product);
        $a = [$product => 0, 0 => 1];
        $remainder = $left % 2.0;
        $b = [$remainder => 0, 0 => 1];
        $divisor = 5.9 % $right;
        $c = [$divisor => 0, 0 => 1];
    }

    function truncatedZero(): void
    {
        $a = [5.9 % 0.9 => 0, 0 => 1];
    }

    function nonfiniteDivisor(): void
    {
        $a = [5.9 % \INF => 0, 0 => 1];
    }

    function nanDivisor(): void
    {
        $a = [5.9 % \NAN => 0, 0 => 1];
    }
}

namespace MissingConstantDuplicateKeys {
    use const MissingConstants\PHP_INT_SIZE;

    function imported(): void
    {
        $a = [PHP_INT_SIZE => 0, 4 => 1, 8 => 2];
    }

    function qualified(): void
    {
        $a = [\MissingConstants\PHP_INT_SIZE => 0, 4 => 1, 8 => 2];
    }
}

namespace ShadowConstantDuplicateKeys {
    use const FullConstantDuplicateKeys\MODULO as PHP_INT_SIZE_ALIAS;
    const PHP_INT_SIZE = 16;
    const INF = 3;
    const NAN = 5;

    function keys(): void
    {
        \PHPStan\dumpType(PHP_INT_SIZE);
        \PHPStan\dumpType(INF);
        \PHPStan\dumpType(NAN);
        \PHPStan\dumpType(PHP_INT_SIZE_ALIAS);
        $a = [PHP_INT_SIZE + 1 => 0, 17 => 1, 5 => 2, 9 => 3];
        $b = [INF + NAN => 0, 8 => 1, 0 => 2];
    }
}
