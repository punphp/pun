<?php
namespace CoalesceOffsetProbes;

function absentOffsets(): void {
    $record = ['a' => 1, 'b' => null, 'c' => ['d' => 1]];
    echo $record['x'] ?? 0;
    \PHPStan\dumpType($record);
    echo $record['x']['y'] ?? 0;
    \PHPStan\dumpType($record);
    echo $record['c']['e'] ?? 0;
    $record['a'] ??= 1;
    \PHPStan\dumpType($record);
}

/** @param array<string, int> $map
 * @param list<int> $list */
function generalOffsets(array $map, array $list): void {
    echo $map['a'] ?? 0;
    echo $map[1] ?? 0;
    \PHPStan\dumpType($map);
    echo $list[0] ?? 0;
    echo $list['a'] ?? 0;
    \PHPStan\dumpType($list);
}

/** @param array{first: int|null, second: int} $record */
function requiredNullable(array $record): void {
    echo $record['second'];
    $key = 'first';
    if (!isset($record[$key])) {
        \PHPStan\dumpType($record);
        \PHPStan\dumpType($record['first']);
    } else {
        \PHPStan\dumpType($record['first']);
    }
}

/** @param array{first?: int|null, second: int} $record */
function optionalNullable(array $record): void {
    echo $record['first'] ?? 0;
    if (!isset($record['first'])) {
        \PHPStan\dumpType($record);
    }
}

/** @param array{first?: int, second: int} $record */
function optionalInteger(array $record): void {
    echo $record['first'] ?? 0;
    $key = 'first';
    if (!isset($record[$key])) {
        \PHPStan\dumpType($record);
    }
}

/** @param array<string, int|null> $map
 * @param list<int|null> $list */
function generalNullable(array $map, array $list): void {
    echo $list[0] ?? 0;
    if (!isset($map['a'])) {
        \PHPStan\dumpType($map);
    }
    if (!isset($list[0])) {
        \PHPStan\dumpType($list);
    }
}

/** @param array{nested?: array{value: int|null}} $record */
function optionalParent(array $record): void {
    echo $record['nested']['value'] ?? 0;
    \PHPStan\dumpType($record);
    if (!isset($record['nested']['value'])) {
        \PHPStan\dumpType($record);
    }
}
