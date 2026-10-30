<?php

namespace ForeachEntryJoins;

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
 * @template T of \stdClass
 * @param T $iterator
 */
function objectTemplateIterator($iterator): void
{
    foreach ($iterator as $item) {}
    \PHPStan\dumpType($iterator);
}

function mixedIterator(mixed $iterator): void
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
    compact('value');
    if (isset($value)) {
        throw new \Exception();
    }
    echo $value;
    compact('value');
}

/** @param list<\stdClass|null> $items */
function nullableForeachValue(array $items): void
{
    if (isset($value)) {
        echo $value;
    }
    foreach ($items as $value) {}
    if (isset($value)) {
        throw new \Exception();
    }
    echo $value;
    compact('value');
}
