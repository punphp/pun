<?php
namespace ConstantForeachBoundaries;

function integer(int $value): void {}

/** @param array<string, mixed> $record */
function breaking(array $record, bool $stop): void {
    foreach (['first', 'second'] as $column) {
        if ($stop) { break; }
        $record[$column] = 7;
    }
    integer($record['second']);
}

/** @param array<string, mixed> $record */
function emptyKeys(array $record): void {
    foreach ([] as $column) {
        $record[$column] = 7;
    }
    integer($record['second']);
}

/** @param array<string, mixed> $record
 * @param list<string> $columns */
function dynamicKeys(array $record, array $columns): void {
    foreach ($columns as $column) {
        $record[$column] = 7;
    }
    integer($record['second']);
}

/** @param array<string, mixed> $record */
function references(array $record): void {
    $columns = ['first', 'second'];
    foreach ($columns as &$column) {
        if (!is_int($record[$column])) { $record[$column] = 7; }
    }
    integer($record['second']);
}

/** @param list<array<string, mixed>> $records */
function outerContinue(array $records, bool $skip): void {
    foreach ($records as $record) {
        foreach (['first', 'second'] as $column) {
            if ($skip) { continue 2; }
            $record[$column] = 7;
        }
        integer($record['second']);
    }
}

/** @param array<string, mixed> $record
 * @return array<string, mixed> */
function requiredContinue(array $record): array {
    foreach (['first', 'second'] as $column) {
        $record[$column] = 11;
        continue;
    }
    \PHPStan\dumpType($record['first']);
    \PHPStan\dumpType($record['second']);
    return $record;
}

/** @param array<string, mixed> $record
 * @return array<string, mixed> */
function nestedKeys(array $record): array {
    foreach (['first', 'second'] as $column) {
        foreach ([1, 2] as $value) {
            $record[$column] = $value;
        }
    }
    \PHPStan\dumpType($record['first']);
    \PHPStan\dumpType($record['second']);
    return $record;
}

/** @param array<string, mixed> $record
 * @return array<string, mixed> */
function nestedBudget(array $record): array {
    foreach (['first', 'second', 'third'] as $column) {
        foreach ([1, 2, 3] as $value) {
            $record[$column] = $value;
        }
    }
    \PHPStan\dumpType($record['first']);
    \PHPStan\dumpType($record['second']);
    return $record;
}
