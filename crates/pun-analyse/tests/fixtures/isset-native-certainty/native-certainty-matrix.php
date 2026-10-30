<?php

namespace IssetNativeCertainty;

/** @param array{required: int, nullable: ?int, optional?: int, nested: array{leaf: int}} $shape */
function shape(array $shape): void
{
    var_dump(isset($shape['required']), isset($shape['nullable']), isset($shape['optional']));
    var_dump(isset($shape['nested']['leaf']), isset($shape['absent']));
    var_dump($shape['required'] ?? 0, $shape['nested']['leaf'] ?? 0);
    $shape['required'] ??= 0;
}

/** @param string $value */
function variable(?string $value, int $native): void
{
    var_dump(isset($value), isset($native));
    var_dump($value ?? '', $native ?? 0);
    $value ??= '';
    $native ??= 0;
}

/** @param array{first: int, second: ?int} $shape */
function dynamic(array $shape, string $key): void
{
    $key = 'first';
    var_dump(isset($shape[$key]));
    var_dump($shape[$key = 'second'] ?? 0);
    $shape[$key = 'first'] ??= 0;
}

/** @template T */
class Box
{
    /** @var T */
    public $value;
    public int $defaulted = 1;
    public ?int $nullable = null;
    public int $uninitialized;
}

/** @param Box<int> $box */
function properties(Box $box): void
{
    $name = 'value';
    var_dump(isset($box->value), isset($box->$name));
    var_dump($box->value ?? 0, $box->$name ?? 0);
    $box->value ??= 0;
    var_dump(isset($box->defaulted), isset($box->nullable), isset($box->uninitialized));
}

class NativeProperties
{
    public int $value = 1;
    public ?int $nullable = null;
}

/** @param NativeProperties $box */
function nullsafe(?NativeProperties $box): void
{
    var_dump(isset($box?->value), isset($box?->nullable));
    var_dump($box?->value ?? 0, $box?->nullable ?? 0);
}

function assignments(?string $value): void
{
    $value = 'actual';
    var_dump(isset($value), $value ?? '');
    $value ??= '';
    $shape = ['key' => 1];
    var_dump(isset($shape['key']), $shape['key'] ?? 0);
}

function unrelated(NativeProperties $box): void
{
    $box->value = 'wrong';
}

function documentedCopy(?string $value): void
{
    /** @var string $copy */
    $copy = $value;
    var_dump(isset($copy), $copy ?? '');
    $copy ??= '';
}

function refined(?string $value): void
{
    if ($value === null) {
        return;
    }
    var_dump(isset($value), $value ?? '');
    $value ??= '';
}
