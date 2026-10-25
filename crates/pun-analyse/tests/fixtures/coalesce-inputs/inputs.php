<?php

namespace CoalesceInputs;

function absent(): void
{
    $undefined ??= $undefined ?? 1;
    \PHPStan\dumpType($undefined);
    compact('undefined');
}

function counterfactual(): void
{
    $value = 1;
    $value ??= $value ?? 2;
    \PHPStan\dumpType($value);
    compact('value');
}

function nullable(?int $value): void
{
    $value ??= $value ?? 1;
    \PHPStan\dumpType($value);
    compact('value');
}

/** @param array<int, array{1: int, 2: int, 3: float}> $rows */
function loop(array $rows): array
{
    $items = [];
    foreach ($rows as $row) {
        $key = $row[1];
        $month = $row[2];
        $items[$key][$month]['foo'] ??= 5;
        $items[$key][$month]['bar'] ??= 5;
        $items[$key][$month]['amount'] ??= 0.0;
    }
    return $items;
}

/** @param array{bar?: int} $items */
function missing(array $items): void
{
    $items['foo'] ??= 5;
    \PHPStan\dumpType($items['foo']);
}

/** @param array{foo: int} $items */
function required(array $items): void
{
    $items['foo'] ??= 5;
    \PHPStan\dumpType($items['foo']);
}

function indexOnce(): void
{
    $items = [];
    $index = 0;
    $items[$index++] ??= 5;
    \PHPStan\dumpType($index);
}

function possible(bool $flag): void
{
    if ($flag) {
        $value = 1;
    }
    $value ??= 2;
    \PHPStan\dumpType($value);
}

function guaranteedRight(): void
{
    $copy = $undefined ?? ($side = 3);
    \PHPStan\dumpType($copy);
    \PHPStan\dumpType($side);
    compact('side');
}

function conditionalRight(?int $value): void
{
    $copy = $value ?? ($side = 3);
    \PHPStan\dumpType($copy);
    \PHPStan\dumpType($side);
    compact('side');
}
