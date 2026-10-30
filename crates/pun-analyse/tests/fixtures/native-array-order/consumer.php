<?php
namespace NativeArrayOrderCertaintyConsumer;

function insertionOrder(): bool
{
    $shape = ['second' => null, 'first' => 1];
    return isset($shape['first']);
}

function nestedCopy(): bool
{
    $shape = ['z' => ['second' => null, 'first' => 1], 'a' => 2];
    $copy = $shape['z'];
    return isset($copy['first']);
}

/** @param array{0: object, 1: non-empty-string}&callable $callback */
function typedCallable(array $callback): bool
{
    return isset($callback[0]);
}

/** @param array{0: object, 1: non-empty-string}&callable $callback */
function untypedCallable($callback): bool
{
    return isset($callback[0]);
}

function integerKeys(): bool
{
    $shape = [1 => 'second', 0 => 'first'];
    return isset($shape[0]);
}

function duplicateReplacement(): bool
{
    $shape = ['second' => 10, 'first' => 1, 'second' => null];
    return isset($shape['first']);
}

function unkeyedList(): bool
{
    $shape = [3, 1];
    return isset($shape[0]);
}

function overwrittenCopy(): bool
{
    $shape = ['second' => null, 'first' => 1];
    $copy = $shape;
    $shape = ['a' => 2, 'z' => 3];
    return isset($copy['first'], $shape['a']);
}

/** @param array{0: int} $unknown */
function genericDependency(array $unknown): bool
{
    $shape = ['z' => $unknown, 'a' => 2];
    return isset($shape['z'][0]);
}

/** @param array{0: array{second: null, first: int}} $unknown */
function nestedGenericDependency(array $unknown): bool
{
    $shape = ['z' => $unknown, 'a' => 2];
    return isset($shape['z'][0]['first']);
}

function unknownKey(string $key): bool
{
    $shape = [$key => 1];
    return isset($shape[$key]);
}

function escapedReference(): bool
{
    $shape = ['only' => 1];
    $reference =& $shape;
    return isset($reference['only']);
}

function annotatedCallable($unknown): bool
{
    /** @var array{0: object, 1: non-empty-string}&callable $callback */
    $callback = $unknown;
    return isset($callback[0]);
}
