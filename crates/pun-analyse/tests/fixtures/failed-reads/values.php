<?php

namespace FailedReads;

function missingOffsets(): void
{
    $array = ['kept' => 1];
    \PHPStan\dumpType($array['missing']);
    $copy = $array['missing'];
    \PHPStan\dumpType($copy);
    \PHPStan\dumpType($array['kept']);
    $list = [1];
    \PHPStan\dumpType($list[2]);
    $empty = [];
    \PHPStan\dumpType($empty[0]);
}

/** @param array{a: int, n: null}|null $array */
function nullableParent(?array $array): void
{
    \PHPStan\dumpType($array['a']);
    \PHPStan\dumpType($array['n']);
    \PHPStan\dumpType($array['missing']);
    $copy = $array['a'];
    \PHPStan\dumpType($copy);
    unset($array['a']['bad']);
}

/** @param array{a: 1}|array{b: 2} $array */
function unionParent(array $array): void
{
    \PHPStan\dumpType($array['a']);
    \PHPStan\dumpType($array['b']);
    \PHPStan\dumpType($array['missing']);
}

function invalidParents(): void
{
    $null = null;
    \PHPStan\dumpType($null[0]);
    \PHPStan\dumpType($null);
    $false = false;
    \PHPStan\dumpType($false[0]);
    \PHPStan\dumpType($false);
    $int = 1;
    \PHPStan\dumpType($int[0]);
    \PHPStan\dumpType($int);
}

function absentRoots(): void
{
    unset($missing['key']);
    \PHPStan\dumpType($missing);
    unset($missing);
    $array = [];
    unset($array);
    unset($array['key']);
    \PHPStan\dumpType($array);
    unset($array);
}

function realNull(): void
{
    $null = null;
    unset($null['key']);
    \PHPStan\dumpType($null);
    $array = ['a' => null];
    unset($array['a']['key']);
    \PHPStan\dumpType($array['a']);
}

/** @param array{0: int}|null $array */
function indexOnce(?array $array): void
{
    $index = 0;
    $value = $array[$index++];
    \PHPStan\dumpType($value);
    \PHPStan\dumpType($index);
}

/** @param array{a: int}|null $array */
function coalesceControl(?array $array): void
{
    \PHPStan\dumpType($array['a'] ?? false);
    \PHPStan\dumpType($array['missing'] ?? 3);
}
