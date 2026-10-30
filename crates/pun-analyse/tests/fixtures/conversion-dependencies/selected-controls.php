<?php

declare(strict_types=1);

namespace ConversionSelected;

final class First
{
    public const ARGS = ['z' => 11, 'a' => 'first'];
}

final class Second
{
    public const ARGS = ['other' => 'second', 'tail' => 22];
}

class Receiver
{
    public function left(int $z, string $a): void
    {
    }

    public function right(string $other, int $tail): void
    {
    }
}

class Child extends Receiver
{
    public function left(int $renamedZ, string $a): void
    {
    }

    public function right(string $renamedOther, int $tail): void
    {
    }
}

function correlated(Receiver $target, bool $flag): void
{
    $row = $flag
        ? ['method' => 'left', 'holder' => First::class]
        : ['method' => 'right', 'holder' => Second::class];
    $target->{$row['method']}(...$row['holder']::ARGS); // selected
}

function literalControls(Receiver $target): void
{
    $target->left(...First::ARGS); // selected
    $target->right(...Second::ARGS); // selected
}

function constantObservation(): array
{
    $holder = First::class;
    $value = $holder::ARGS;
    $value; // unavailable:class-constant-value
    return $value;
}
