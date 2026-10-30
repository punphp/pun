<?php

declare(strict_types=1);

namespace ConversionDependencies;

function implicitOffset(): bool
{
    $source = [['z' => 7, 'a' => null], ['b' => false, 'y' => 42]];
    $copy = $source[0];
    $copy; // observe:implicit-leaf
    return isset($copy['a']);
}

function copiedExplicitOffset(): bool
{
    $source = ['z' => ['last' => false, 'first' => 23], 'a' => ['other' => null]];
    $key = 'z';
    $root = $source;
    $leaf = $root[$key];
    $leaf; // observe:explicit-leaf
    return isset($leaf['first']);
}

/** @param array{array{a: null, z: 'doc'}} $source */
function genericNativeDependency(array $source): bool
{
    $root = [$source, ['a' => 99, 'z' => false]];
    $leaf = $root[0][0];
    $leaf; // observe:generic-leaf
    return isset($leaf['z']);
}

/** @param 0 $key */
function documentedKey(int $key): void
{
    $source = [['z' => 7, 'a' => null], 99];
    $key; // observe:documented-key
    $leaf = $source[$key];
    $leaf; // observe:documented-key-leaf
}

function unknownKey(string $key): void
{
    $source = [$key => ['z' => 7, 'a' => null]];
    $source; // unavailable:unknown-key-source
}

function consume(array $value): void
{
}

function unsupportedContinuation(): void
{
    $source = [['z' => 7, 'a' => null], ['b' => false, 'y' => 42]];
    $before = $source[0];
    $before; // observe:before-array-call
    consume($source);
    $after = $source[0];
    $after; // unavailable:after-array-call
}
