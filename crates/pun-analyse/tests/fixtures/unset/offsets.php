<?php

namespace UnsetOffsets;

function certainty(bool $condition, string $name): void
{
    unset($missing);
    unset($missing['key']);
    if ($condition) {
        $maybe = [];
    }
    unset($maybe);
    $defined = [];
    unset($defined, $defined);
    unset($defined);
    unset($$name);
    unset($_GET['key']);
}

function scalars(): void
{
    $integer = 7;
    unset($integer['bad']);
    $null = null;
    unset($null[0]);
    $bool = false;
    unset($bool[0]);
    $float = 2.5;
    unset($float[0]);
    $text = 'abc';
    unset($text[-4]);
    $text = 'abc';
    unset($text[-3]);
    $text = 'abc';
    unset($text[2]);
    $text = 'abc';
    unset($text[3]);
    $text = 'abc';
    unset($text['0']);
    $text = 'abc';
    unset($text[false]);
}

/**
 * @param array{a?: int, 1: int} $shape
 * @param 'a'|'absent' $either
 * @param 'absent'|'missing' $neither
 * @param array<int, string> $ints
 * @param list<string> $list
 * @param non-empty-list<string> $nonEmpty
 * @param iterable<string, int> $iterable
 * @param array{a: int}|array{b: int} $union
 */
function arrays(array $shape, string $either, string $neither, array $ints, array $list, array $nonEmpty, iterable $iterable, array $union, string $unknown): void
{
    unset($shape[$either]);
    unset($shape[$neither]);
    unset($shape[1]);
    unset($shape['1']);
    unset($shape[true]);
    unset($shape[1.9]);
    unset($shape[null]);
    unset($shape[$unknown]);
    unset($ints['bad']);
    unset($ints[$unknown]);
    unset($list[-1]);
    unset($nonEmpty[0]);
    unset($iterable[1]);
    unset($union['a']);
    unset($union['missing']);
    $nested = ['a' => ['b' => 1]];
    unset(
        $nested['a']['b']['c']
    );
}

/** @template T of array{a: int}
 * @param T $array
 */
function template(array $array): void
{
    unset($array['missing']);
}

/** @param \ArrayAccess<string, int> $access */
function objects(\ArrayAccess $access, \stdClass $open, Closed $closed, mixed $mixed, object $object): void
{
    unset($access[0]);
    unset($access['key']);
    unset($open[0]);
    unset($closed[0]);
    unset($mixed[0]);
    unset($object[0]);
}

final class Closed {}

unset($globalUnknown);

function argumentSideEffect(): void
{
    $array = ['a' => 1];
    unset($array[$created = 'a'], $created);
}
