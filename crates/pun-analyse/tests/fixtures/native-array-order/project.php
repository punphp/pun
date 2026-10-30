<?php
namespace NativeArrayOrderProject;

function insertionOrder(): void
{
    $shape = ['second' => null, 'first' => 1];
    if (is_array($shape)) {}
}

function copiedAndNested(): void
{
    $shape = ['z' => ['second' => null, 'first' => 1], 'a' => 2];
    $copy = $shape;
    if (is_array($copy)) {}
    if (is_array($shape['z'])) {}
}

function duplicateAndIntegerKeys(): void
{
    $shape = ['second' => 10, 'first' => 1, 'second' => null];
    if (is_array($shape)) {}
    $reversed = [1 => 'second', 0 => 'first'];
    if (is_array($reversed)) {}
}

function overwrittenCopy(): void
{
    $shape = ['second' => null, 'first' => 1];
    $copy = $shape;
    $shape = ['a' => 2, 'z' => 3];
    if (is_array($shape)) {}
    if (is_array($copy)) {}
}

/** @param array{0: int} $unknown */
function genericNativeDependency(array $unknown): void
{
    $shape = ['z' => $unknown, 'a' => 2];
    if (is_array($shape['z'])) {}
    $list = [3, 1];
    if (is_array($list)) {}
}
