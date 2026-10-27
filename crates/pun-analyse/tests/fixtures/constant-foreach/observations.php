<?php
namespace ConstantForeachObservations;

function integer(int $value): void {}

/** @param list<array<string, mixed>> $records */
function conditional(array $records): void {
    foreach ($records as $record) {
        foreach (['first', 'second', 'third'] as $column) {
            integer($record[$column]);
            \PHPStan\dumpType($column);
            if (!array_key_exists($column, $record) || !is_int($record[$column])) {
                $record[$column] = 31;
            }
        }
        integer($record['first']);
        integer($record['second']);
        integer($record['third']);
    }
}

/** @param list<array<string, mixed>> $records */
function continuing(array $records): void {
    foreach ($records as $record) {
        foreach (['first', 'second', 'third'] as $column) {
            integer($record[$column]);
            \PHPStan\dumpType($column);
            if (array_key_exists($column, $record) && is_int($record[$column])) {
                continue;
            }
            $record[$column] = 47;
        }
        integer($record['first']);
        integer($record['second']);
        integer($record['third']);
    }
}

/** @param array<string, mixed> $record
 * @return array<string, mixed> */
function noInnerBackEdge(array $record): array {
    foreach ([0, 1] as $pass) {
        foreach (['first', 'second'] as $column) {
            integer($record[$column]);
            continue 2;
        }
    }
    return $record;
}
