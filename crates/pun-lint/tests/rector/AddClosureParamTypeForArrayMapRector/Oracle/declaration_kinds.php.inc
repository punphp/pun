<?php

namespace Probe;

interface Left
{
}

interface Right
{
}

class Node
{
    /**
     * @return list<static>
     */
    public static function many(): array
    {
        return [];
    }

    /**
     * @return list<$this>
     */
    public function selves(): array
    {
        return [];
    }

    public function mapSelves()
    {
        return array_map(function ($node) {
            return $node;
        }, $this->selves());
    }
}

final class Leaf extends Node
{
}

/**
 * @template T of int
 * @template U
 * @param list<T> $ints
 * @param list<U> $anything
 * @param list<array-key> $keys
 * @param list<scalar> $scalars
 * @param list<true> $trues
 * @param list<false> $falses
 * @param list<int|true> $intOrTrue
 * @param list<int|false|null> $falsy
 * @param list<(Left&Right)|null> $dnf
 * @param list<object|Leaf> $objectOrLeaf
 * @param list<int>|list<string> $twoLists
 * @param array{a: int}|array{b: string} $twoShapes
 * @param list<int>|array $listOrArray
 * @param array{} $empty
 * @param array{a: int, ...<string, string>} $unsealed
 * @param list{int, string} $pair
 */
function run(
    array $ints,
    array $anything,
    array $keys,
    array $scalars,
    array $trues,
    array $falses,
    array $intOrTrue,
    array $falsy,
    array $dnf,
    array $objectOrLeaf,
    array $twoLists,
    array $twoShapes,
    array $listOrArray,
    array $empty,
    array $unsealed,
    array $pair,
) {
    array_map(function ($v) { return $v; }, $ints);
    array_map(function ($v) { return $v; }, $anything);
    array_map(function ($v) { return $v; }, $keys);
    array_map(function ($v) { return $v; }, $scalars);
    array_map(function ($v) { return $v; }, $trues);
    array_map(function ($v) { return $v; }, $falses);
    array_map(function ($v) { return $v; }, $intOrTrue);
    array_map(function ($v) { return $v; }, $falsy);
    array_map(function ($v) { return $v; }, $dnf);
    array_map(function ($v) { return $v; }, $objectOrLeaf);
    array_map(function ($v) { return $v; }, $twoLists);
    array_map(function ($v) { return $v; }, $twoShapes);
    array_map(function ($v) { return $v; }, $listOrArray);
    array_map(function ($v) { return $v; }, $empty);
    array_map(function ($v) { return $v; }, $unsealed);
    array_map(function ($v) { return $v; }, $pair);
    array_map(function ($v) { return $v; }, Leaf::many());
}

/**
 * @return list<int>
 */
function ints(): array
{
    return [];
}

$top = array_map(function ($v) {
    return $v;
}, ints());
-----
<?php

namespace Probe;

interface Left
{
}

interface Right
{
}

class Node
{
    /**
     * @return list<static>
     */
    public static function many(): array
    {
        return [];
    }

    /**
     * @return list<$this>
     */
    public function selves(): array
    {
        return [];
    }

    public function mapSelves()
    {
        return array_map(function (self $node) {
            return $node;
        }, $this->selves());
    }
}

final class Leaf extends Node
{
}

/**
 * @template T of int
 * @template U
 * @param list<T> $ints
 * @param list<U> $anything
 * @param list<array-key> $keys
 * @param list<scalar> $scalars
 * @param list<true> $trues
 * @param list<false> $falses
 * @param list<int|true> $intOrTrue
 * @param list<int|false|null> $falsy
 * @param list<(Left&Right)|null> $dnf
 * @param list<object|Leaf> $objectOrLeaf
 * @param list<int>|list<string> $twoLists
 * @param array{a: int}|array{b: string} $twoShapes
 * @param list<int>|array $listOrArray
 * @param array{} $empty
 * @param array{a: int, ...<string, string>} $unsealed
 * @param list{int, string} $pair
 */
function run(
    array $ints,
    array $anything,
    array $keys,
    array $scalars,
    array $trues,
    array $falses,
    array $intOrTrue,
    array $falsy,
    array $dnf,
    array $objectOrLeaf,
    array $twoLists,
    array $twoShapes,
    array $listOrArray,
    array $empty,
    array $unsealed,
    array $pair,
) {
    array_map(function (int $v) { return $v; }, $ints);
    array_map(function ($v) { return $v; }, $anything);
    array_map(function (int|string $v) { return $v; }, $keys);
    array_map(function (int|float|string|bool $v) { return $v; }, $scalars);
    array_map(function (true $v) { return $v; }, $trues);
    array_map(function (false $v) { return $v; }, $falses);
    array_map(function (int|true $v) { return $v; }, $intOrTrue);
    array_map(function (int|false|null $v) { return $v; }, $falsy);
    array_map(function ((\Probe\Left&\Probe\Right)|null $v) { return $v; }, $dnf);
    array_map(function (object $v) { return $v; }, $objectOrLeaf);
    array_map(function (int|string $v) { return $v; }, $twoLists);
    array_map(function ($v) { return $v; }, $twoShapes);
    array_map(function ($v) { return $v; }, $listOrArray);
    array_map(function ($v) { return $v; }, $empty);
    array_map(function ($v) { return $v; }, $unsealed);
    array_map(function ($v) { return $v; }, $pair);
    array_map(function (\Probe\Leaf $v) { return $v; }, Leaf::many());
}

/**
 * @return list<int>
 */
function ints(): array
{
    return [];
}

$top = array_map(function (int $v) {
    return $v;
}, ints());
