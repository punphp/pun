<?php

$forward = function (bool $b) {
    return $b ? 1.5 : 'x';
};
$reverse = function (bool $b) {
    return $b ? 'x' : 1.5;
};
$nested = function (bool $a, bool $b) {
    return $a ? 0 : ($b ? true : []);
};
$nestedReverse = function (bool $a, bool $b) {
    return $a ? true : ($b ? 0 : []);
};
$separateReturns = function (bool $b) {
    if ($b) {
        return 'x';
    }
    return 1.5;
};
$writtenForward = function (float|string $value) {
    return $value;
};
$writtenReverse = function (string|float $value) {
    return $value;
};
-----
<?php

$forward = function (bool $b): float|string {
    return $b ? 1.5 : 'x';
};
$reverse = function (bool $b): string|float {
    return $b ? 'x' : 1.5;
};
$nested = function (bool $a, bool $b): array|int|true {
    return $a ? 0 : ($b ? true : []);
};
$nestedReverse = function (bool $a, bool $b): array|true|int {
    return $a ? true : ($b ? 0 : []);
};
$separateReturns = function (bool $b): string|float {
    if ($b) {
        return 'x';
    }
    return 1.5;
};
$writtenForward = function (float|string $value): float|string {
    return $value;
};
$writtenReverse = function (string|float $value): string|float {
    return $value;
};
