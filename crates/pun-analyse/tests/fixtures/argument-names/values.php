<?php declare(strict_types=1);

namespace ArgumentNames;

/**
 * @param callable(int): int $unnamed
 * @param callable(int $value): int $named
 * @param callable(int ...): int $unnamedVariadic
 * @param callable(int ...$values): int $namedVariadic
 */
function values(callable $unnamed, callable $named, callable $unnamedVariadic, callable $namedVariadic): void
{
    $unnamed('wrong');
    $named('wrong');
    $named(value: 'wrong');
    $unnamedVariadic('wrong', 1, 'wrong');
    $namedVariadic('wrong', 1, 'wrong');
    $unnamedVariadic(extra: 'wrong');
    $namedVariadic(extra: 'wrong');

    $unnamed(1);
    $named(1);
    $named(value: 1);
    $unnamedVariadic(1, 2, 3);
    $namedVariadic(1, 2, 3);
    $unnamedVariadic(extra: 1);
    $namedVariadic(extra: 1);
}
