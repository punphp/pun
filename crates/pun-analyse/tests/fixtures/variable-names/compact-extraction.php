<?php

namespace CompactExtraction;

function stringBuiltin(bool $condition): void
{
    $name = get_include_path();
    if ($name === 'missing' || $name === false) {
        compact($name);
        if ($condition) {
            $missing = 1;
        }
        compact(
            $name,
            $name,
        );
    }
    compact(get_include_path());
}

function arrayBuiltin(bool $condition): void
{
    $names = scandir(__DIR__);
    if ($names === ['arrayMissing', 'repeatedMissing', 'repeatedMissing'] || $names === false) {
        compact($names);
        if ($condition) {
            $arrayMissing = $repeatedMissing = 1;
        }
        compact($names);
    }
    compact(scandir(__DIR__));
}

/** @param array<int, 'missing'>&callable $names */
function callableGeneric($names): void
{
    compact($names);
}

/** @param array{0: class-string, 1: 'methodMissing'}&callable $names */
function callableOffsets($names): void
{
    compact($names);
}

/** @param array<int, string>&callable $names */
function callableNarrowedOffsets($names): void
{
    if ($names[1] === 'narrowedMissing') {
        compact($names);
    }
}

/** @param array<int, '0'|'methodMissing'>&callable $names */
function callableNonFalsy($names): void
{
    compact($names);
}

/** @param 'literalMissing'|string $name */
function broadString($name): void
{
    compact($name);
}

/** @param array{'literalMissing'}|array<string> $names */
function broadArray($names): void
{
    compact($names);
}

/** @param 'missing'|false $ordinary */
function unrelatedAndReplacedSources($ordinary, bool $condition): void
{
    $name = get_include_path();
    compact($ordinary);
    $name = $ordinary;
    compact($name);

    $names = get_include_path();
    if ($names === 'scalarMissing' || $names === false) {
        if ($condition) {
            $names = ['arrayMissing'];
        }
        // PHPStan makes this merge an ordinary union: neither extraction domain is complete.
        compact($names);
    }
}

/** @param array<int, 'methodMissing'>&callable $names */
function concreteWritesWin($names): void
{
    $names = ['existing'];
    $existing = 1;
    compact($names);
}

class CallableNames
{
    public static function classMethodMissing(): void {}
}

/** @param array<int, 'CompactExtraction\CallableNames'|'classMethodMissing'>&callable $names */
function callableProjectClass($names): void
{
    compact($names);
}

/** @param array{0: 'CompactExtraction\CallableNames', 1: 'classMethodMissing'}&callable $names */
function callableProjectShape($names): void
{
    compact($names);
}
