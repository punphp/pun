<?php
namespace RefinedOffsetHistory;

// Repeated valid probes do not establish presence; rejected string/boolean probes
// must not specify a differently spelled integer-offset expression.
function strings(string $value): void {
    echo $value['1'] ?? 'absent';
    echo isset($value[1]);
    echo $value[1] ?? 'absent';
    echo $value['01'] ?? 'absent';
    echo isset($value[1]);
    echo $value[0] ?? 'absent';
    echo isset($value[0]);
    echo $value[0] ?? 'absent';
    echo $value[false] ?? 'absent';
    echo isset($value[0]);
    echo $value[-1] ?? 'absent';
    echo isset($value[-1]);
}

/** @param array{0?: string, 1: ?string} $value */
function arrays(array $value): void {
    echo $value['0'] ?? 'absent';
    echo isset($value[0]);
    echo $value[0] ?? 'absent';
    echo $value['1'] ?? 'absent';
    echo isset($value[1]);
    echo $value[1] ?? 'absent';
    $value['0'] ??= 'written';
    echo isset($value[0]);
    echo $value[0] ?? 'absent';
    echo isset($value['0']);
    echo $value['0'] ?? 'absent';
    $value['1'] ??= 'written';
    echo isset($value[1]);
    echo $value[1] ?? 'absent';
    echo isset($value['1']);
    echo $value['1'] ?? 'absent';
}

/** @param array{0: null, 1: string} $value */
function nullableAndRequired(array $value): void {
    echo $value['0'] ?? 'absent';
    echo isset($value[0]);
    echo $value[0] ?? 'absent';
    echo $value['1'] ?? 'absent';
    echo isset($value[1]);
    echo $value[1] ?? 'absent';
}

/** @param array<int, string>|string $value */
function mixed(array|string $value): void {
    echo $value['0'] ?? 'absent';
    echo isset($value[0]);
    echo $value[0] ?? 'absent';
}

function indexes(string $string): void {
    $i = 0;
    echo $string[$i++] ?? 'absent';
    echo $i;
    echo isset($string[0]);
    echo isset($string[1]);
    $values = ['first', 'second'];
    $j = 0;
    echo $values[$j++] ?? 'absent';
    echo $j;
    echo isset($values[0]);
    echo isset($values[1]);
}
