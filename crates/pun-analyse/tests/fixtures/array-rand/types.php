<?php

namespace ArrayRand;

use function PHPStan\dumpType;

/** @param int<2, max> $many */
function emptyArray(int $num, int $many): void
{
    $arr = [];
    $x = array_rand($arr);
    $y = array_rand($arr, $num);
    dumpType($x);
    dumpType($y);
    dumpType(array_rand($arr, 1));
    dumpType(array_rand($arr, 2));
    dumpType(array_rand($arr, 0));
    dumpType(array_rand($arr, -5));
    dumpType(array_rand($arr, $many));
    dumpType(array_rand(num: 2, array: []));
    dumpType(array_rand(array_slice([1], 0, 0)));
}

/**
 * @param non-empty-array<string, int> $strings
 * @param array<int, string> $integers
 * @param list<string> $list
 */
function generalArrays(array $strings, array $integers, array $list): void
{
    dumpType(array_rand($strings));
    dumpType(array_rand($integers));
    dumpType(array_rand($list));
}

function shapes(): void
{
    $array = ['b' => 1, 'a' => 2, 5 => 3];
    dumpType(array_rand($array));
    dumpType(array_rand(num: 1, array: ['only' => 1]));
}

function maybeEmpty(bool $empty): void
{
    $values = $empty ? [] : ['x' => 1];
    dumpType(array_rand($values));
}

function unknownArray(mixed $values): void
{
    echo array_rand($values);
}

/**
 * @template T of array<string, int>
 * @param T $values
 */
function genericArray(array $values): void
{
    echo array_rand($values);
}

function insufficient(): void
{
    $key = array_rand();
}
