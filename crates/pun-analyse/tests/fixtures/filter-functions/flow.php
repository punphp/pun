<?php

namespace FilterProvidersFlow;

function needs_int(int $value): void {}

function explicitness(mixed $value, int $flags, int $filter): void
{
    // The extension's unknown-state result is implicit mixed, not the native explicit mixed return.
    needs_int(filter_var($value, $filter));
    needs_int(filter_var($value, FILTER_VALIDATE_INT, $flags));
    needs_int(filter_input(INPUT_POST, 'x', FILTER_VALIDATE_INT, $flags));
    // Do not suppress an explicit input or the precise validation failure union.
    needs_int($value);
    needs_int(filter_var($value, FILTER_VALIDATE_INT));
    // Compatibility controls at declined boundaries: invalid array + explicit mixed default.
    // These do not claim that default casts or PHP 8.5 throwing are implemented.
    needs_int(filter_var([], FILTER_DEFAULT, ['options' => ['default' => $value]]));
    needs_int(filter_var([], FILTER_VALIDATE_INT, ['flags' => 268435456, 'options' => ['default' => $value]]));
    $number = filter_input(INPUT_POST, 'x', FILTER_VALIDATE_INT);
    if ($number !== false && $number !== null) {
        needs_int($number);
    }
    $ips = filter_var(['127.0.0.1', 'bad'], FILTER_VALIDATE_IP, FILTER_REQUIRE_ARRAY);
    echo count(array_filter($ips));
}
