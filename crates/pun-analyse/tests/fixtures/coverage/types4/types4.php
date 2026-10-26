<?php

namespace Types4;

/**
 * @template T of int|string
 * @param T $a
 * @param array<T> $b
 * @return T
 */
function pick($a, array $b)
{
    return $a;
}

/**
 * @template T
 * @param list<T> $xs
 * @param callable(T): bool $f
 */
function keep(array $xs, callable $f): void
{
}

/** @param array<mixed> $a */
function array_any(array $a): bool
{
    return $a !== [];
}

/**
 * @param list<int> $ints
 * @param list<list<int>>|array{list<int>} $lists
 */
function calls(array $ints, array $lists): void
{
    $_ = pick(1.5, [1]);
    $_ = pick('a', ['b']);
    keep([1, 2], fn (string $x): bool => true);
    $_ = array_filter([1, 2], fn (string $k): bool => true, ARRAY_FILTER_USE_KEY);
    $_ = array_filter(['a' => 1], fn (int $v, int $k): bool => true, ARRAY_FILTER_USE_BOTH);
    $_ = array_map(fn (string $a, string $b): string => $a, ...[[1], [2]]);
    $pairs = rand() > 0 ? [[1], [2]] : [[3]];
    $_ = array_map(fn (string $a, string $b): string => $a, ...$pairs);
    $_ = array_map(fn (): string => 'a', ...$lists);
    $_ = array_walk($ints, fn (int $v, int $k, string $x) => null, 1);
    $_ = array_find([1], fn (string $v): bool => true);
    $_ = implode([1]);
    $_ = implode(array: [1]);
    $_ = array_any([1]);
}

function terminatedArrayCallback(): void
{
    array_find(throw new \RuntimeException(), static fn (int $value): bool => true);
    $_ = array_find([1], static fn (int $value): bool => true);
}
