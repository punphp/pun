<?php declare(strict_types=1);

namespace BuiltinCallableVariants;

use function PHPStan\dumpType;

/** @param callable(): int $callback */
function zero(callable $callback): void {}
/** @param callable(int): int $callback */
function one(callable $callback): void {}
/** @param callable(int, int): int $callback */
function two(callable $callback): void {}

function overloaded(): void
{
    zero(mt_rand(...));
    one(mt_rand(...));
    two(mt_rand(...));
    zero('mt_rand');
    zero('mt_rand'(...));
    zero(rand(...));
    $source = mt_rand(...);
    $copy = $source;
    zero($copy);
    $copy = static fn(): int => 1;
    zero($copy);
    dumpType($source, mt_rand(...), strtok(...));
}

function local(int $input = 1): int { return $input; }
function controls(): void
{
    zero(local(...));
    zero(static fn(): int => 1);
    two(local(...));
    zero(strlen(...));
    dumpType(local(...), strlen(...), sprintf(...), preg_match(...));
    dumpType(date_diff(...), chr(...));
    $sort = sort(...);
    $sort([3, 1]);
    $items = [3, 1];
    $sort($items);
}

namespace BuiltinCallableVariants\Shadow;

function mt_rand(): string { return 'local'; }
function control(): void
{
    \PHPStan\dumpType(mt_rand(...));
    \BuiltinCallableVariants\zero(mt_rand(...));
}
