<?php

namespace ConverterRemainder;

/**
 * @template T
 * @param T $input
 * @param callable(T): void $callback
 */
function apply($input, callable $callback): void
{
}

function extraPositional(): void
{
    apply('kept', function ($value): void {
        \PHPStan\dumpType($value);
    }, 99);
}

function extraNamed(): void
{
    apply(callback: function ($value): void {
        \PHPStan\dumpType($value);
    }, input: 42, unknown: false);
}

/** @param array{z: int, a: string, ...<string, int|string>} $shape */
function positionalMetadata(array $shape): void
{
    apply($shape, function ($value): void {
        \PHPStan\dumpType($value);
        \PHPStan\dumpPhpDocType($value);
    }, ['other' => false]);
}

/** @param array{z: string, a: int, ...<string, int|string>} $shape */
function namedMetadata(array $shape): void
{
    apply(unknown: ['different' => false], callback: function ($value): void {
        \PHPStan\dumpType($value);
        \PHPStan\dumpPhpDocType($value);
    }, input: $shape);
}
