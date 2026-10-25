<?php declare(strict_types=1);

namespace ArgumentNames;

/**
 * @param callable(string &): string $unnamed
 * @param callable(string &$value): string $named
 * @param callable(string &...): string $unnamedVariadic
 * @param callable(string &...$values): string $namedVariadic
 */
function references(callable $unnamed, callable $named, callable $unnamedVariadic, callable $namedVariadic): void
{
    $unnamed('first');
    $named('first');
    $named(value: 'first');
    $unnamedVariadic('first', 'second');
    $namedVariadic('first', 'second');
    $unnamedVariadic(extra: 'first');
    $namedVariadic(extra: 'first');

    $first = 'first';
    $second = 'second';
    $unnamed($first);
    $named($first);
    $named(value: $first);
    $unnamedVariadic($first, $second);
    $namedVariadic($first, $second);
    $unnamedVariadic(extra: $first);
    $namedVariadic(extra: $first);
}

/**
 * @param callable(string &): string $unnamed
 * @param callable(string &$value): string $named
 * @param callable(string &...): string $unnamedVariadic
 * @param callable(string &...$values): string $namedVariadic
 */
function pipes(callable $unnamed, callable $named, callable $unnamedVariadic, callable $namedVariadic): void
{
    'first' |> $unnamed;
    'first' |> $named;
    'first' |> $unnamedVariadic;
    'first' |> $namedVariadic;
}
