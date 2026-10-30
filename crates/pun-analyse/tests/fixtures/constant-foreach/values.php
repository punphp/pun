<?php
namespace ConstantForeachValues;

function integer(int $value): void {}
/** @param 0 $value */
function zero(int $value): void {}

/** @param list<array{first: int, second: string}> $records */
function conditional(array $records): void {
    foreach ($records as $record) {
        foreach (['first', 'second', 'missing'] as $column) {
            if (!array_key_exists($column, $record) || !is_int($record[$column])) {
                $record[$column] = 17;
            }
        }
        \PHPStan\dumpType($record['first']);
        \PHPStan\dumpType($record['second']);
        \PHPStan\dumpType($record['missing']);
        integer($record['first']);
        integer($record['second']);
        integer($record['missing']);
    }
}

/** @param list<array{first: int, second: string}> $records */
function continuing(array $records): void {
    foreach ($records as $record) {
        foreach (['first', 'second', 'missing'] as $column) {
            if (array_key_exists($column, $record) && is_int($record[$column])) {
                continue;
            }
            $record[$column] = 23;
        }
        \PHPStan\dumpType($record['first']);
        \PHPStan\dumpType($record['second']);
        \PHPStan\dumpType($record['missing']);
        integer($record['first']);
        integer($record['second']);
        integer($record['missing']);
    }
}

/** @param list<array<string, mixed>> $records */
function asymmetric(array $records): void {
    foreach ($records as $record) {
        foreach (['first', 'second'] as $index => $column) {
            $record[$column] = $index;
        }
        \PHPStan\dumpType($record['first']);
        \PHPStan\dumpType($record['second']);
        zero($record['first']);
        zero($record['second']);
    }
}

/** @param list<array<string, mixed>> $records */
function stringFallback(array $records): void {
    foreach ($records as $record) {
        foreach (['first', 'second'] as $column) {
            if (array_key_exists($column, $record) && is_int($record[$column])) {
                continue;
            }
            $record[$column] = 'fallback';
        }
        \PHPStan\dumpType($record['first']);
        \PHPStan\dumpType($record['second']);
        integer($record['first']);
        integer($record['second']);
    }
}
