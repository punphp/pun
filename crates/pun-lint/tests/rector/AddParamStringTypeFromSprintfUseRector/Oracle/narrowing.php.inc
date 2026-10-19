<?php

namespace App;

function oneArm($code)
{
    return match ($code) {
        1 => 'one',
        default => sprintf('%s', $code),
    };
}

function twoArms($code)
{
    return match ($code) {
        1, 2 => 'small',
        default => sprintf('%s', $code),
    };
}

function twoCases($code)
{
    switch ($code) {
        case 1:
            return 'one';
        case 2:
            return 'two';
        default:
            return sprintf('%s', $code);
    }
}

function firstArm($code)
{
    return match ($code) {
        'a' => sprintf('%s', $code),
        'b', 'c' => 'other',
        default => 'x',
    };
}

function onOther($code, $other)
{
    return match ($other) {
        1, 2 => 'small',
        default => sprintf('%s', $code),
    };
}
-----
<?php

namespace App;

function oneArm(string $code)
{
    return match ($code) {
        1 => 'one',
        default => sprintf('%s', $code),
    };
}

function twoArms($code)
{
    return match ($code) {
        1, 2 => 'small',
        default => sprintf('%s', $code),
    };
}

function twoCases(string $code)
{
    switch ($code) {
        case 1:
            return 'one';
        case 2:
            return 'two';
        default:
            return sprintf('%s', $code);
    }
}

function firstArm(string $code)
{
    return match ($code) {
        'a' => sprintf('%s', $code),
        'b', 'c' => 'other',
        default => 'x',
    };
}

function onOther(string $code, $other)
{
    return match ($other) {
        1, 2 => 'small',
        default => sprintf('%s', $code),
    };
}
