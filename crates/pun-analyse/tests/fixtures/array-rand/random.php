<?php

namespace RandomReturns;

use function PHPStan\dumpType;

function literals(): void
{
    dumpType(rand());
    dumpType(mt_rand());
    dumpType(rand(-7, 11));
    dumpType(rand(11, -7));
    dumpType(mt_rand(-3, 13));
    dumpType(random_int(-17, 5));
    dumpType(random_int(4, 4));
    dumpType(rand(min: -5, max: 19));
    dumpType(mt_rand(max: 23, min: -2));
    dumpType(random_int(max: 31, min: -11));
    dumpType(random_int(PHP_INT_MIN, PHP_INT_MAX));
    dumpType(random_int(PHP_INT_MAX, PHP_INT_MAX));
    dumpType(random_int(PHP_INT_MIN, PHP_INT_MIN));
}

/**
 * @param -9|-2 $lower
 * @param 3|14 $upper
 * @param int<min, -5> $unboundedLower
 * @param int<12, max> $unboundedUpper
 */
function domains(int $lower, int $upper, int $unboundedLower, int $unboundedUpper, int $unknown): void
{
    dumpType(rand($lower, $upper));
    dumpType(rand($upper, $lower));
    dumpType(mt_rand($lower, $upper));
    dumpType(random_int($lower, $upper));
    dumpType(random_int($unboundedLower, 19));
    dumpType(random_int(-7, $unboundedUpper));
    dumpType(rand($unboundedLower, 19));
    dumpType(rand(-7, $unboundedUpper));
    dumpType(random_int($unknown, $unknown));
    dumpType(rand($unknown, -3));
}

function nested(): void
{
    dumpType(random_int(random_int(-19, -3), random_int(8, 27)));
    dumpType(rand(random_int(8, 27), random_int(-19, -3)));
    $num = rand(1, 5);
    dumpType($num);
    if ($num == 1) {
        dumpType($num);
    } else {
        dumpType($num);
        dumpType(array_rand(['a' => 1, 'b' => 2], $num));
    }
}

/** @param int<min, -4> $lower */
function machineMinimum(int $lower): void
{
    dumpType(random_int($lower, PHP_INT_MIN));
    dumpType(random_int($lower, -9223372036854775808));
}

/** @param int<7, max> $upper */
function machineMaximum(int $upper): void
{
    dumpType(random_int(PHP_INT_MAX, $upper));
    dumpType(random_int(9223372036854775807, $upper));
}

function insufficient(): void
{
    $a = rand(3);
    $b = mt_rand(5);
    $c = random_int();
    $d = random_int(7);
}

/** @param 2|5 $lower */
function coercions(int $lower, bool $bool, string $text, float $float, mixed $unknown): void
{
    dumpType(rand(' -4.8suffix', '1.2e1'));
    dumpType(random_int(-3.9, 7.8));
    dumpType(rand(null, true));
    dumpType(rand($bool, 9));
    dumpType(rand($text, 9));
    dumpType(rand($float, 9));
    dumpType(rand($unknown, 9));
    dumpType(rand($lower, 13));
    dumpType(rand([], [1]));
    dumpType(rand(new \stdClass(), 9));
    dumpType(rand(['named' => 1], 9));
    dumpType(rand([], 9));
    dumpType(random_int(9223372036854775808.0, 5));
    dumpType(random_int(-18446744073709549568.0, 3000));
    dumpType(random_int(INF, 7));
    dumpType(random_int(NAN, 7));
}

/**
 * @template N of int<3, 11>
 * @param N $number
 * @param array<string, int> $array
 * @param list<never> $bottomList
 * @param array{optional?: int} $optional
 */
function convertedDomains(int $number, array $array, array $bottomList, array $optional): void
{
    dumpType(rand($number, 19));
    dumpType(rand($array, 9));
    dumpType(rand($bottomList, 9));
    dumpType(rand($optional, 9));
}

/**
 * @template N of 3
 * @template S of '12'
 * @param N $number
 * @param S $string
 */
function literalTemplates(int $number, string $string): void
{
    dumpType(rand($number, 19));
    dumpType(rand($string, 19));
}

/**
 * @template N of 2|7
 * @template B of true
 * @param N $number
 * @param B $bool
 */
function compoundTemplates(int $number, bool $bool): void
{
    dumpType(rand($number, 19));
    dumpType(rand($bool, 19));
    dumpType(array_rand(['only' => 1], $number));
    dumpType(random_int($number, 19));
    dumpType(random_int(-13, $number));
}

function malformedNamedArguments(): void
{
    dumpType(rand(unknown: 3));
    dumpType(mt_rand(extra: 3));
    dumpType(rand());
    dumpType(mt_rand());
}
