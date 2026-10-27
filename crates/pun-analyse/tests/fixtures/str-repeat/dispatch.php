<?php
namespace RepeatDispatch;
use function PHPStan\dumpType;

function dispatch(): void
{
    dumpType(\STR_REPEAT('a', 2));
    dumpType('str_repeat'('a', 2));
    dumpType((\str_repeat(...))('a', 2));
    $callable = \str_repeat(...);
    dumpType($callable('a', 2));
    $name = 'str_repeat';
    dumpType($name('a', 2));
    dumpType(987);
}

namespace RepeatShadow;
function str_repeat(string $value, int $count): int { return 42; }
\PHPStan\dumpType(str_repeat('a', 2));
