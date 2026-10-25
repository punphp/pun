<?php

namespace ArrayRandFull;

use function PHPStan\dumpType;

/**
 * @param non-empty-array<string, int> $strings
 * @param array<int, string> $integers
 * @param list<string> $list
 * @param int<2, max> $many
 * @param 1|3 $oneOrMany
 */
function general(array $strings, array $integers, array $list, int $many, int $unknown, int $oneOrMany): void
{
    dumpType(array_rand($strings));
    dumpType(array_rand($strings, $many));
    dumpType(array_rand($strings, $unknown));
    dumpType(array_rand($integers, $unknown));
    dumpType(array_rand($list, 2));
    dumpType(array_rand($list, $oneOrMany));
}

function shapes(int $unknown): void
{
    $values = ['z' => 1, 'a' => 2, 17 => 3];
    dumpType(array_rand($values));
    dumpType(array_rand($values, 2));
    dumpType(array_rand($values, $unknown));
    dumpType(array_rand([10, 20, 30]));
    dumpType(array_rand(num: 2, array: ['second' => 2, 'first' => 1]));
    dumpType(array_rand(['nested' => array_rand(['inside' => 1], 2)]));
    dumpType(array_rand(array_slice(['only' => 1], 0), 2));
}

/** @param array{last?: int} $optional */
function unions(bool $empty, bool $choose, array $optional, int $num): void
{
    $values = $empty ? [] : ['named' => 1];
    dumpType(array_rand($values));
    dumpType(array_rand($values, $num));
    $values = $choose ? ['left' => 1] : [9 => 2];
    dumpType(array_rand($values, 2));
    dumpType(array_rand($optional));
}

/**
 * @template T of array<string, int>
 * @param T $values
 */
function templateArray(array $values, int $num): void
{
    dumpType(array_rand($values));
    dumpType(array_rand($values, 2));
    dumpType(array_rand($values, $num));
}

/**
 * @template K of array-key
 * @param array<K, string> $values
 */
function templateKeys(array $values): void
{
    dumpType(array_rand($values));
    dumpType(array_rand($values, 2));
}

/** @param list<never> $values */
function bottomList(array $values): void
{
    dumpType(array_rand($values));
}

/** @param iterable<string, int> $iterable */
function otherInputs(mixed $unknown, iterable $iterable): void
{
    dumpType(array_rand($unknown));
    dumpType(array_rand($unknown, 2));
    dumpType(array_rand($iterable));
    dumpType(array_rand(123));
}

function countBoundaries(): void
{
    dumpType(array_rand(['only' => 1], 999));
    dumpType(array_rand(['only' => 1], 0));
    dumpType(array_rand(['only' => 1], -3));
    dumpType(array_rand(['only' => 1], true));
    dumpType(array_rand(['only' => 1], '2'));
    array_rand();
}

/**
 * @template N of int<2, 9>
 * @param N $count
 */
function templateCount(int $count): void
{
    dumpType(array_rand(['only' => 1], $count));
}

/** @param never $count */
function impossibleCount($count): void
{
    dumpType(array_rand(['only' => 1], $count));
}

/**
 * @template One of 1
 * @template Many of 3
 * @param One $one
 * @param Many $many
 */
function literalTemplateCounts(int $one, int $many): void
{
    dumpType(array_rand(['only' => 1], $one));
    dumpType(array_rand(['only' => 1], $many));
}
