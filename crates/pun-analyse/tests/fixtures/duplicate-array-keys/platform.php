<?php

namespace PlatformDuplicateKeys {
    use const PHP_INT_MAX as MAXIMUM;

    function copies(): void
    {
        $max = PHP_INT_MAX;
        $copy = $max;
        \PHPStan\dumpType($max);
        $a = [2147483647 => 0, 9223372036854775807 => 1, $max => 2];
        $b = [2147483647 => 0, 9223372036854775807 => 1, $copy => 2];
        $min = PHP_INT_MIN;
        \PHPStan\dumpType($min);
        $c = [-2147483648 => 0, -9223372036854775808 => 1, $min => 2];
        $size = PHP_INT_SIZE;
        \PHPStan\dumpType($size);
        $d = [4 => 0, 8 => 1, $size => 2];
        $alias = MAXIMUM;
        $e = [2147483647 => 0, 9223372036854775807 => 1, $alias => 2];
        $qualified = \PHP_INT_MAX;
        $f = [2147483647 => 0, 9223372036854775807 => 1, $qualified => 2];
        assert($max === 9223372036854775807);
        \PHPStan\dumpType($max);
        $g = [2147483647 => 0, 9223372036854775807 => 1, $max => 2];
        assert($min === -2147483648);
        $h = [-2147483648 => 0, -9223372036854775808 => 1, $min => 2];
        assert($size === 4);
        $i = [4 => 0, 8 => 1, $size => 2];
        $max = 7;
        $j = [2147483647 => 0, 9223372036854775807 => 1, $max => 2];
        $k = [7 => 0, $max => 1];
        $other = 2147483647;
        $l = [2147483647 => 0, 9223372036854775807 => 1, $other => 2];
    }

    /** @param 2147483647|9223372036854775807 $key */
    function narrowed(int $key): void
    {
        assert($key === 2147483647);
        $a = [2147483647 => 0, 9223372036854775807 => 1, $key => 2];
    }
}

namespace LocalPlatformDuplicateKeys {
    const PHP_INT_MAX = 9;

    function shadow(): void
    {
        $local = PHP_INT_MAX;
        $a = [9 => 0, 2147483647 => 1, 9223372036854775807 => 2, $local => 3];
    }
}

namespace {
    function globalMaximumCopy(): void
    {
        $max = PHP_INT_MAX;
        \PHPStan\dumpType($max);
        $a = [2147483647 => 0, 9223372036854775807 => 1, $max => 2];
    }
}
