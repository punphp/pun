<?php

namespace GlobalInitializerDefines;

function beforeDeclarations(): void
{
    \PHPStan\dumpType(SPLIT);
    \PHPStan\dumpType(COALESCE);
    \PHPStan\dumpType(MIDDLE);
}

define('GlobalInitializerDefines\\SPLIT', true ? 1 : 4);
define('GlobalInitializerDefines\\COALESCE', 0 ?? 1);
define('GlobalInitializerDefines\\MIDDLE', true ? 0 : 1);

function afterDeclarations(): void
{
    \PHPStan\dumpType(SPLIT);
    \PHPStan\dumpType(COALESCE);
    \PHPStan\dumpType(MIDDLE);
    unrelated('GlobalInitializerDefines\\SPLIT', 9);
    \PHPStan\dumpType(SPLIT);
}

function unrelated(string $name, int $value): void {}

#[\Attribute(SPLIT)] class InvalidSplit {}
#[\Attribute(COALESCE)] class InvalidCoalesce {}
#[\Attribute(MIDDLE)] class ReflectionNotFlow {}

#[InvalidSplit, InvalidCoalesce, ReflectionNotFlow]
class Consumer {}
