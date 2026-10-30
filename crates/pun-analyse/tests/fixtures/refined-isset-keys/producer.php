<?php
namespace RefinedOffsetProducer;

function observe(): void {}

function strings(string $value): void {
    echo isset($value['0']);
    observe(); // holders: $value['0'] | $value[0]
    echo $value['0'] ?? 'absent';
    observe(); // holders: $value['0'] | $value[0]
    echo $value[0] ?? 'absent';
    observe(); // holders: $value['0'] | $value[0]
    echo $value['1'] ?? 'absent';
    observe(); // holders: $value['1'] | $value[1]
    echo $value[false] ?? 'absent';
    observe(); // holders: $value[false] | $value[0]
    echo $value["2"] ?? 'absent';
    observe(); // holders: $value['2'] | $value[2]
}

/** @param array{0?: string, 1: ?string, 2: null, 3: string} $value */
function arrays(array $value): void {
    echo $value['0'] ?? 'absent';
    observe(); // holders: $value['0'] | $value[0]
    echo $value['1'] ?? 'absent';
    observe(); // holders: $value['1'] | $value[1]
    echo $value['2'] ?? 'absent';
    observe(); // holders: $value['2'] | $value[2]
    echo $value['3'] ?? 'absent';
    observe(); // holders: $value['3'] | $value[3]
    $value['0'] ??= 'written';
    observe(); // holders: $value['0'] | $value[0]
    echo $value[1] ?? 'absent';
    echo $value['1'] ?? 'absent';
    observe(); // holders: $value['1'] | $value[1]
    echo $value["3"] ?? 'absent';
    observe(); // holders: $value['3'] | $value[3]
}

/** @param array{0: ?string} $value */
function variableKey(array $value): void {
    $key = 0;
    echo $value[$key] ?? 'absent';
    observe(); // holders: $value[$key] | $value[0]
}

/** @param array{0: array{0: ?string}} $value */
function nested(array $value): void {
    echo $value['0']['0'] ?? 'absent';
    observe(); // holders: $value['0']['0'] | $value[0][0] | $value['0'] | $value[0]
}

function indexes(string $value): void {
    $i = 0;
    echo $value[$i++] ?? 'absent';
    observe(); // holders: $value[$i++] | $value[$i] | $value[0] | $i
    $array = ['first', 'second'];
    $j = 0;
    echo $array[$j++] ?? 'absent';
    observe(); // holders: $array[$j++] | $array[$j] | $array[0] | $j
}

/** @param array{0: ?string, 1: ?string, '': ?string, -1: ?string} $value */
function rawArrayKeys(array $value): void {
    echo $value[false] ?? 'absent';
    observe(); // holders: $value[false] | $value[0]
    echo $value[true] ?? 'absent';
    observe(); // holders: $value[true] | $value[1]
    echo $value[null] ?? 'absent';
    observe(); // holders: $value[null] | $value['']
    echo $value[-1] ?? 'absent';
    observe(); // holders: $value[-1]
    echo $value[+1] ?? 'absent';
    observe(); // holders: $value[+1] | $value[1]
}

/** @param array{0?: ?string} $value */
function nullableWrite(array $value): void {
    $value['0'] ??= null;
    observe(); // holders: $value['0'] | $value[0]
}

/** @param array{0?: array{0: ?string}} $value */
function optionalParent(array $value): void {
    echo $value['0']['0'] ?? 'absent';
    observe(); // holders: $value['0']['0'] | $value[0][0] | $value['0'] | $value[0]
}

/** @param array{0: ?string} $value */
function lexicalKeys(array $value): void {
    echo $value[0x0] ?? 'absent';
    observe(); // holders: $value[0] | $value['0']
    echo $value[FALSE] ?? 'absent';
    observe(); // holders: $value[FALSE] | $value[false] | $value[0]
}
