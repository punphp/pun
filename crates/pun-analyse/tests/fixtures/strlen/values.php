<?php

namespace StrlenReturns;

use function PHPStan\dumpType;

function values(): void
{
    dumpType(strlen('abc'));
    dumpType(strlen(''));
    dumpType(strlen("é😀"));
    dumpType(strlen("\xff\x00\x80"));
    dumpType(strlen(string: "\xffé"));
    dumpType(strlen(true));
    dumpType(strlen(false));
    dumpType(strlen(null));
    dumpType(strlen(123));
    dumpType(strlen(-123));
    dumpType(strlen(PHP_INT_MIN));
    dumpType(strlen(1.0));
    dumpType(strlen(1.23));
    dumpType(strlen(0.0));
    dumpType(strlen(-0.0));
    dumpType(strlen(1.0e20));
    dumpType(strlen(1.0e-5));
    dumpType(strlen(1.2345678901234567));
    dumpType(strlen(INF));
    dumpType(strlen(-INF));
    dumpType(strlen(NAN));
    dumpType(strlen(\stdClass::class));
    $bytes = "\xff\xfe";
    $length = strlen($bytes);
    dumpType($length);
    dumpType(strlen(strlen('abc')));
}

function named(): void
{
    dumpType(strlen(string: 'named'));
    dumpType(strlen(...['string' => 'unpacked']));
    dumpType(strlen(string: 'first', extra: 1));
    dumpType(strlen(extra: 1, string: 'reordered'));
    strlen();
}

/**
 * @param 0.0|-0.0 $positiveFirst
 * @param -0.0|0.0 $negativeFirst
 * @param 1.23|2.0 $finite
 * @param 1.23|float $broad
 */
function floatUnions($positiveFirst, $negativeFirst, $finite, $broad): void
{
    dumpType(strlen($positiveFirst));
    dumpType(strlen($negativeFirst));
    dumpType(strlen($finite));
    dumpType(strlen($broad));
}
