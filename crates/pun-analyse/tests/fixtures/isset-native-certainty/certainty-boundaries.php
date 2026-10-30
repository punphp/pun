<?php

namespace IssetCertaintyBoundaries;

/** @param array{required: int, nullable: ?int, optional?: int, nested: array{leaf: int}} $shape */
function required(array $shape): bool
{
    return isset($shape['required']);
}

/** @param array{required: int, nullable: ?int, optional?: int, nested: array{leaf: int}} $shape */
function nullable(array $shape): bool
{
    return isset($shape['nullable']);
}

/** @param array{required: int, nullable: ?int, optional?: int, nested: array{leaf: int}} $shape */
function optional(array $shape): bool
{
    return isset($shape['optional']);
}

/** @param array{required: int, nullable: ?int, optional?: int, nested: array{leaf: int}} $shape */
function nested(array $shape): bool
{
    return isset($shape['nested']['leaf']);
}

/** @param array{required: int, nullable: ?int, optional?: int, nested: array{leaf: int}} $shape */
function absent(array $shape): bool
{
    return isset($shape['absent']);
}

/** @param array{first: int, second: ?int} $shape */
function incoming(array $shape, string $key): bool
{
    return isset($shape[$key = 'first'], $shape[$key = 'second']);
}

/** @param array{required: int} $shape */
function coalesce(array $shape): int
{
    return $shape['required'] ?? 0;
}

/** @param array{required: int} $shape */
function assignCoalesce(array $shape): void
{
    $shape['required'] ??= 0;
}

/** @param string $value */
function variable(?string $value): bool
{
    return isset($value);
}

function nativeVariable(int $value): bool
{
    return isset($value);
}

/** @param string $value */
function variableCoalesce(?string $value): string
{
    return $value ?? '';
}

/** @param string $value */
function variableAssignCoalesce(?string $value): void
{
    $value ??= '';
}

function annotatedCopy(?string $value): bool
{
    /** @var string $copy */
    $copy = $value;
    return isset($copy);
}

function actualAssignment(?string $value): bool
{
    $value = 'actual';
    return isset($value);
}

function actualArray(): bool
{
    $shape = ['second' => null, 'first' => 1];
    return isset($shape['first']);
}

function refined(?string $value): bool
{
    if ($value === null) {
        return false;
    }
    return isset($value);
}

/** @template T */
class Box
{
    /** @var T */
    public $value;
    public int $defaulted = 1;
    public int $uninitialized;
    public ?int $nullable = null;

    public function initialized(): bool
    {
        $this->uninitialized = 1;
        return isset($this->uninitialized);
    }
}

/** @param Box<int> $box */
function generic(Box $box): bool
{
    return isset($box->value);
}

/** @param Box<int> $box */
function dynamicGeneric(Box $box): bool
{
    $name = 'value';
    return isset($box->$name);
}

/** @param Box<int> $box */
function genericCoalesce(Box $box): int
{
    return $box->value ?? 0;
}

/** @param Box<int> $box */
function genericAssignCoalesce(Box $box): void
{
    $box->value ??= 0;
}

function defaulted(Box $box): bool
{
    return isset($box->defaulted);
}

function uninitialized(Box $box): bool
{
    return isset($box->uninitialized);
}

function nullableProperty(Box $box): bool
{
    return isset($box->nullable);
}

function arbitraryExpression(): string
{
    return 'native' ?? 'fallback';
}

function unrelated(Box $box): void
{
    $box->defaulted = 'wrong';
}

function afterUnsupportedCall(int $value): bool
{
    var_dump($value);
    return isset($value);
}

function afterUnset(): bool
{
    $shape = ['present' => 1];
    unset($shape['present']);
    return isset($shape['present']);
}

/** @param array{0: object, 1: non-empty-string}&callable $callback */
function documentedCallableArray(array $callback): bool
{
    return isset($callback[0]);
}

/** @param array{0: object, 1: non-empty-string}&callable $callback */
function documentedUntypedCallableArray($callback): bool
{
    return isset($callback[0]);
}
