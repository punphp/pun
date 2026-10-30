<?php
namespace ConstantForeachScopes;

function integer(int $value): void {}

/** @param array<string, mixed> $record */
function singleKey(array $record): void {
    $key = 'first';
    if (is_int($record[$key])) {
        integer($record['first']);
        integer($record['second']);
    }
}

/** @param array<int, mixed> $record */
function integerKey(array $record): void {
    $key = 0;
    if (is_int($record[$key])) {
        integer($record[0]);
        integer($record[1]);
    }
}

/** @param array<string, mixed> $record */
function unionKey(array $record, bool $choice): void {
    $key = $choice ? 'first' : 'second';
    if (is_int($record[$key])) {
        integer($record['first']);
        integer($record['second']);
    }
}

/** @param array{first: int|null, second: int} $record */
function nullableKey(array $record): void {
    $key = 'first';
    if (!isset($record[$key])) {
        \PHPStan\dumpType($record['first']);
        integer($record['second']);
    } else {
        integer($record['first']);
    }
}

/** @param array{first?: int, second: int} $record */
function optionalKey(array $record): void {
    $key = 'first';
    if (!isset($record[$key])) {
        \PHPStan\dumpType($record);
    } else {
        integer($record['first']);
    }
}
