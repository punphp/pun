<?php

final class WrittenOrder
{
    public function forward(): float|string
    {
        return 1.5;
    }

    public function reverse(): string|float
    {
        return 'x';
    }
}

$forward = fn (bool $b) => $b ? 1.5 : 'x';
$reverse = fn (bool $b) => $b ? 'x' : 1.5;
$nested = fn (bool $a, bool $b) => $a ? 0 : ($b ? true : []);
$nestedReverse = fn (bool $a, bool $b) => $a ? true : ($b ? 0 : []);
$parenthesized = fn (bool $b) => ($b ? 1.5 : 'x');
$elvis = fn (float $n) => $n ?: 'x';
$alwaysTrue = fn (bool $b) => true ? ($b ? 'x' : 1.5) : 0;
$alwaysFalse = fn (bool $b) => false ? 0 : ($b ? 1.5 : 'x');
$writtenForward = fn (float|string $value) => $value;
$writtenReverse = fn (string|float $value) => $value;
$receiverForward = fn (WrittenOrder $receiver) => $receiver->forward();
$receiverReverse = fn (WrittenOrder $receiver) => $receiver->reverse();
-----
<?php

final class WrittenOrder
{
    public function forward(): float|string
    {
        return 1.5;
    }

    public function reverse(): string|float
    {
        return 'x';
    }
}

$forward = fn (bool $b): float|string => $b ? 1.5 : 'x';
$reverse = fn (bool $b): string|float => $b ? 'x' : 1.5;
$nested = fn (bool $a, bool $b): array|int|bool => $a ? 0 : ($b ? true : []);
$nestedReverse = fn (bool $a, bool $b): array|bool|int => $a ? true : ($b ? 0 : []);
$parenthesized = fn (bool $b): float|string => ($b ? 1.5 : 'x');
$elvis = fn (float $n): float|string => $n ?: 'x';
$alwaysTrue = fn (bool $b): string|float => true ? ($b ? 'x' : 1.5) : 0;
$alwaysFalse = fn (bool $b): float|string => false ? 0 : ($b ? 1.5 : 'x');
$writtenForward = fn (float|string $value): float|string => $value;
$writtenReverse = fn (string|float $value): string|float => $value;
$receiverForward = fn (WrittenOrder $receiver): float|string => $receiver->forward();
$receiverReverse = fn (WrittenOrder $receiver): string|float => $receiver->reverse();
