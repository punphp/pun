<?php

namespace UnsealedUnpacking;

function one(int $value): void {}
function two(int $first, int $second): void {}

/**
 * @param array{0: int} $sealed
 * @param array{0: string, ...<int, bool>} $open
 * @param array{0?: int, ...<int, bool>} $optional
 * @param array{...<int, bool>} $possiblyEmpty
 * @param array{} $empty
 */
function calls(array $sealed, array $open, array $optional, array $possiblyEmpty, array $empty, bool $choose): void
{
    $union = $choose ? $sealed : $open;
    one(...$open);
    one(...$union);
    one(...$optional);
    one(...$possiblyEmpty);
    one(...$empty);
    one(...$sealed);
    two(...$open);
    two(1, ...$open);
    two(...$empty, ...$open);
    two(...$sealed);
}
