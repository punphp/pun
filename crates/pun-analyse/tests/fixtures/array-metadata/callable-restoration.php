<?php

namespace ArrayMetadata;

use function PHPStan\dumpType;

class CallableReceiver
{
    public static function invoke(): void {}
}

/**
 * @param array&callable&iterable $callback
 * @param array{0: 'ArrayMetadata\CallableReceiver', 1: 'invoke'}&callable&iterable $exact
 */
function mixedFallback($callback, $exact): void
{
    dumpType($callback, $exact);
    $copy = $callback;
    $exactCopy = $exact;
    dumpType($copy, $exactCopy);
    $callback = [42];
    $exact = [42];
    dumpType($callback, $copy, $exact, $exactCopy);
}

/** @param array&callable&iterable $callback */
function arrayFallback(array $callback): void
{
    dumpType($callback);
    $copy = $callback;
    dumpType($copy);
    $callback = [42];
    dumpType($callback, $copy);
}

/**
 * @template T of (array&(callable(value-of<array>...):value-of<array>)&iterable)
 * @param T $callback
 */
function implicitCallableBound($callback): void
{
    dumpType($callback);
    $copy = $callback;
    dumpType($copy);
    $callback = [42];
    dumpType($callback, $copy);
}
