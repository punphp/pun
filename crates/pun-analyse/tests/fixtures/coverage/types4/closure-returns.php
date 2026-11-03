<?php

namespace Cov\ClosureReturns;

use function PHPStan\dumpType;

/** @param mixed $m */
function declared($m, ?int $n): void
{
    dumpType(array_map(fn ($value): string => $value, [1, 2]));
    dumpType(array_map(fn ($value): int => $value, [1, 2]));
    dumpType(array_map(fn (int $value): string => $m, [1, 2]));
    dumpType(array_map(fn (int $value): int => $m, [1, 2]));
    dumpType(array_map(function (int $value): string { return $value; }, [1, 2]));
    dumpType(array_map(function (int $value) use ($m): int { return $m; }, [1, 2]));
    dumpType(array_map(fn (int $value): string => $value, [3 => 1, 'a' => 2]));
    dumpType(array_map(fn ($value) => [$value], [1, 2]));
    dumpType(array_map(null, [1, 2]));
    dumpType(fn (): ?int => 5);
    dumpType(fn (): float => 1);
    dumpType(fn (): int => $n);
    dumpType(function (): int {
    });
}

$mapped = array_map(fn (int $value): string => $value, [1, 2]);
dumpType($mapped);
