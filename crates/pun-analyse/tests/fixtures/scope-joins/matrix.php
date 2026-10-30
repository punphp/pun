<?php

namespace SharedScopeJoins;

function ordinaryIf(bool $pick): void
{
    if ($pick) {
        $shape = ['tag' => 'left', 'value' => 1];
        $created = 1;
    } else {
        $shape = ['tag' => 'right', 'value' => null];
    }
    \PHPStan\dumpType($shape);
    echo $created;
}

function booleanIf(bool $pick, bool $other): void
{
    if ($pick && $other) {
        $shape = ['tag' => 'right', 'value' => null];
    } else {
        $shape = ['tag' => 'left', 'value' => 1];
        $created = 1;
    }
    \PHPStan\dumpType($shape);
    echo $created;
}

function terminatingIf(bool $pick): void
{
    if ($pick) {
        return;
    } else {
        $shape = ['tag' => 'only', 'value' => 1];
    }
    \PHPStan\dumpType($shape);
}

/** @param array<string, int|string> $values */
function shortCircuitAnd(bool $pick, array $values, string $key): void
{
    $shape = ['tag' => 'before', 'value' => 1];
    $pick && ($shape = ['tag' => 'after', 'value' => null]);
    $pick && assert(is_string($values[$key]));
    $pick && ($created = 1);
    \PHPStan\dumpType($shape);
    \PHPStan\dumpType($values[$key]);
    echo $created;
}

/** @param array<string, int|string> $values */
function shortCircuitOr(bool $pick, array $values, string $key): void
{
    $shape = ['tag' => 'before', 'value' => 1];
    $pick || ($shape = ['tag' => 'after', 'value' => null]);
    $pick || assert(is_string($values[$key]));
    $pick || ($created = 1);
    \PHPStan\dumpType($shape);
    \PHPStan\dumpType($values[$key]);
    echo $created;
}

/** @throws \RuntimeException */
function mayThrow(): void {}

/** @param array<string, int|string> $values */
function catchEntry(array $values, string $key): void
{
    $shape = ['tag' => 'before', 'value' => 1];
    try {
        mayThrow();
        $shape = ['tag' => 'after', 'value' => null];
        assert(is_string($values[$key]));
        mayThrow();
    } catch (\RuntimeException $error) {
        \PHPStan\dumpType($shape);
        \PHPStan\dumpType($values[$key]);
    }
}

/** @param array<string, int|string> $values */
function cachedBeforeBothThrows(array $values, string $key): void
{
    assert(is_string($values[$key]));
    try {
        mayThrow();
        mayThrow();
    } catch (\RuntimeException $error) {
        \PHPStan\dumpType($values[$key]);
    }
}

function continuingCatch(): void
{
    $shape = ['tag' => 'before', 'value' => 1];
    try {
        mayThrow();
    } catch (\RuntimeException $error) {
        $shape = ['tag' => 'catch', 'value' => null];
    }
    \PHPStan\dumpType($shape);
}

function finallyEntry(): void
{
    $shape = ['tag' => 'before', 'value' => 1];
    try {
        mayThrow();
    } catch (\RuntimeException $error) {
        $shape = ['tag' => 'catch', 'value' => null];
    } finally {
        \PHPStan\dumpType($shape);
    }
}

function terminatingCatch(): void
{
    $shape = ['tag' => 'before', 'value' => 1];
    try {
        mayThrow();
    } catch (\RuntimeException $error) {
        return;
    }
    \PHPStan\dumpType($shape);
}

function forExit(bool $enter): void
{
    $shape = ['tag' => 'before', 'value' => 1];
    for ($initial = 1; $enter;) {
        $shape = ['tag' => 'body', 'value' => null];
        $created = 1;
        break;
    }
    \PHPStan\dumpType($shape);
    echo $initial, $created;
}

function whileExit(bool $enter): void
{
    $shape = ['tag' => 'before', 'value' => 1];
    while ($enter) {
        $shape = ['tag' => 'body', 'value' => null];
        $created = 1;
        break;
    }
    \PHPStan\dumpType($shape);
    echo $created;
}

/** @param list<int> $items */
function foreachExits(array $items, bool $leave): void
{
    $shape = ['tag' => 'before', 'value' => 1];
    foreach ($items as $item) {
        if ($leave) {
            $shape = ['tag' => 'break', 'value' => null];
            break;
        }
        $shape = ['tag' => 'continue', 'value' => false];
        continue;
    }
    \PHPStan\dumpType($shape);
}

/**
 * @param non-empty-list<int> $items
 * @param array<string, int|string> $values
 */
function foreachProjection(array $items, array $values, string $key): void
{
    $shape = ['tag' => 'entry', 'value' => 1];
    foreach ($items as $item) {
        $shape = ['tag' => 'final', 'value' => null];
        $created = 1;
        assert(is_string($values[$key]));
    }
    \PHPStan\dumpType($shape);
    \PHPStan\dumpType($values[$key]);
    echo $created;
}

/**
 * @param non-empty-list<int> $items
 * @param array<string, int|string> $values
 */
function projectionEntryOnly(array $items, array $values, string $key): void
{
    assert(is_string($values[$key]));
    foreach ($items as $item) {
        unset($values[$key]);
    }
    \PHPStan\dumpType($values[$key]);
}

/**
 * @template T
 * @param T $iterator
 */
function templateIterator($iterator): void
{
    foreach ($iterator as $item) {}
    \PHPStan\dumpType($iterator);
}

/**
 * @template T of list<int>
 * @param T $iterator
 */
function replacedTemplateIterator($iterator): void
{
    foreach ($iterator as $item) {
        $iterator = ['final'];
    }
    \PHPStan\dumpType($iterator);
}

/**
 * @template T of \stdClass
 * @param T $iterator
 */
function objectTemplateIterator($iterator): void
{
    foreach ($iterator as $item) {}
    \PHPStan\dumpType($iterator);
}

/** @param list<\stdClass> $items */
function absentBeforeForeach(array $items): void
{
    if (isset($value)) {
        echo $value;
    }
    foreach ($items as $value) {}
    echo $value;
    if (isset($value)) {
        throw new \Exception();
    }
    echo $value;
    compact('value');
}

/**
 * @template T of array<string, int>
 * @param T $iterator
 */
function replacedKeyedTemplateIterator($iterator): void
{
    foreach ($iterator as $item) {
        $iterator = ['final'];
    }
    \PHPStan\dumpType($iterator);
}

/**
 * @template T of non-empty-list<int>
 * @param T $iterator
 */
function replacedNonEmptyTemplateIterator($iterator): void
{
    foreach ($iterator as $item) {
        $iterator = ['final'];
    }
    \PHPStan\dumpType($iterator);
}
