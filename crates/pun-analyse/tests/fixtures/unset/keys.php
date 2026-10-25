<?php

namespace UnsetKeys;

/** @param array{2: int} $array */
function casts(array $array, bool $bool, float $float): void
{
    unset($array[$bool]);
    unset($array[$float]);
    unset($array[1.0e19]);
    unset($array[-1.0e19]);
    unset($array[INF]);
    unset($array[-INF]);
    unset($array[NAN]);
    unset($array['02']);
    unset($array[[]]);
}

/**
 * @param array{a: int, ...<string, int>} $open
 * @param array{1: int, ...<string, int>} $integer
 * @param array{a: int} $shape
 * @param array{1: int} $numeric
 * @param int|string $key
 * @param mixed $mixed
 */
function shapes(array $open, array $integer, array $shape, array $numeric, int|string $key, mixed $mixed): void
{
    unset($open['other']);
    unset($integer['other']);
    unset($open[10]);
    unset($shape[$key]);
    unset($shape[$mixed]);
    unset($numeric[$key]);
    unset($numeric[$mixed]);
}

/** @param array<string, int> $array */
function stringKeys(array $array, int $integer): void
{
    unset($array[$integer]);
}

function genericString(string $allowed, string $text, int $integer, string $string, mixed $mixed): void
{
    unset($allowed[$integer]);
    unset($text[$string]);
    unset($text[$mixed]);
}

/** @param callable&array $callable */
function callableArray(array $callable): void
{
    unset($callable[2]);
    unset($callable['other']);
    unset($callable[0]);
}

/** @param __benevolent<array{a: int}|int> $array */
function benevolent(array|int $array): void
{
    unset($array['a']);
}

function absentIndex(array $array): void
{
    unset($array[$missingIndex]);
}

function inaccessibleInner(): void
{
    $integer = 7;
    unset($integer['first']['second']);
    unset($absent['first']['second']);
}

/** @return callable&array */
function callableResult(): array { return ['strtoupper', 'invoke']; }

/** @return non-empty-list<int> */
function listResult(): array { return [1]; }

/** @return array{a: int, ...<string, int>} */
function openResult(): array { return ['a' => 1]; }

function returnedArrays(): void
{
    unset(callableResult()[2]);
    unset(callableResult()['other']);
    unset(callableResult()[0]);
    unset(listResult()[0]);
    unset(listResult()[-1]);
    unset(listResult()['other']);
    unset(openResult()['other']);
    unset(openResult()[10]);
    unset(getopt('a')['a']);
}

/**
 * @template T of array{a: int}
 * @param T&non-empty-array<string, int> $array
 */
function templateIntersection(array $array): void
{
    unset($array['a']);
    unset($array['missing']);
}

/**
 * @template T of array<string, int>
 * @param T&array{a: int} $array
 * @param T $original
 */
function shapedTemplateIntersection(array $array, array $original): void
{
    unset($array['missing']);
    unset($array['a']);
}
