<?php

namespace IncomingIsset;

function additions(float|int $seed): void
{
    foreach ([1, 2, 3] as $i) {
        if (!isset($sum)) {
            $sum = 0;
        }
        $sum = $sum + $i;
        compact('sum');
    }
    if (!isset($floatOrInt)) {
        $floatOrInt = $seed;
    }
    $floatOrInt += $i;
    compact('floatOrInt');
}

function possible(bool $flag): void
{
    if ($flag) {
        $value = 1;
    }
    if (!isset($value)) {
        $value = 2;
    }
    echo $value;
    compact('value');
}

function nestedDiagnosticBranch(bool $flag): void
{
    if (isset($missing)) {
        if ($flag) {
            echo $missing;
        } else {
            echo $missing;
        }
    }
    echo $missing;
}

function absent(): void
{
    if (isset($missing)) {
        echo $missing;
        compact('missing');
    }
    echo $missing;
    compact('missing');
}

function unrelated(bool $flag): void
{
    if ($flag) {
        $tag = 'left';
        $left = 1;
    } else {
        $tag = 'right';
        $right = 1;
    }
    if (!isset($initialized)) {
        $initialized = 0;
    }
    if ($tag === 'left') {
        echo $initialized, $left, $right;
        compact('initialized', 'left', 'right');
    }
}

/** @param array<string, list<int>> $groups */
function typedMap(array $groups, string $key): void
{
    if (!isset($groups[$key])) {
        $groups[$key] = [];
    }
    array_unshift($groups[$key], 1);
    \PHPStan\dumpType($groups[$key]);
}

/** @param array<string, array<string, list<int>>> $groups */
function nestedMap(array $groups, string $outer, string $inner): void
{
    if (!isset($groups[$outer][$inner])) {
        $groups[$outer][$inner] = [];
    }
    array_unshift($groups[$outer][$inner], 1);
    \PHPStan\dumpType($groups[$outer][$inner]);
}

function absentArray(): void
{
    if (isset($missing['key'])) {
        echo $missing;
        compact('missing');
    }
    echo $missing;
    compact('missing');
}

function knownNull(): void
{
    $value = null;
    if (!isset($value)) {
        $value = 1;
    }
    echo $value;
    compact('value');
}

/** @param list<\stdClass> $all */
function terminatedIsset(array $all): void
{
    foreach ($all as $value) {
    }
    if (isset($value)) {
        throw new \Exception();
    }
    echo $value;
    compact('value');
}

/** @param list<\stdClass|null> $all */
function nullableTerminatedIsset(array $all): void
{
    foreach ($all as $value) {
    }
    if (isset($value)) {
        throw new \Exception();
    }
    echo $value;
    compact('value');
}

/** @param list<mixed> $all */
function mixedTerminatedIsset(array $all): void
{
    foreach ($all as $value) {
    }
    if (isset($value)) {
        return;
    }
    echo $value;
    compact('value');
}

/** @param list<\stdClass|null> $all */
function initializedTerminatedIsset(array $all): void
{
    $value = null;
    foreach ($all as $value) {
    }
    if (isset($value)) {
        throw new \Exception();
    }
    echo $value;
    compact('value');
}

function impossibleNegativeIsset(): void
{
    $value = 1;
    if (!isset($value)) {
        echo $value;
        compact('value');
    }
    echo $value;
    compact('value');
}

function nonNullMixedNegativeIsset(mixed $value): void
{
    if ($value !== null) {
        if (!isset($value)) {
            echo $value;
            compact('value');
        }
    }
    echo $value;
    compact('value');
}

function nullableMixedNegativeIsset(mixed $value): void
{
    if (!isset($value)) {
        echo $value;
        compact('value');
    }
    echo $value;
    compact('value');
}
