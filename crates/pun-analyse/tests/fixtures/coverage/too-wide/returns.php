<?php

namespace CoverageTooWide;

function nativeUnion(): ?string
{
    return 'a';
}

function generator(): \Generator|int
{
    yield 1;
    return 1;
}

function noReturn(): ?int
{
    echo 1;
}

function bare(bool $b): ?int
{
    if ($b) {
        return;
    }
    return 1;
}

/** @return int|string */
function phpdocOnly(bool $b)
{
    if ($b) {
        return 1;
    }
}

function onlyTrue(): bool
{
    return true;
}

/** @return 1|2 */
function constants(): int
{
    return 1;
}

/** @return ($b is true ? int : string) */
function conditional(bool $b): int|string
{
    return 1;
}

trait T
{
    private function inTrait(): ?int
    {
        return 1;
    }
}

class Base
{
    public function over(): ?int
    {
        return 1;
    }

    public function overNull(): ?int
    {
        return 1;
    }
}

final class Child extends Base
{
    use T;

    public function over(): ?int
    {
        return 1;
    }

    public function overNull(): ?int
    {
        return null;
    }

    private function priv(): int|string|null
    {
        return 1;
    }
}

class Open
{
    public function open(): ?int
    {
        return 1;
    }

    final public function closed(): ?int
    {
        return 1;
    }

    /** @final */
    public function tagged(): ?int
    {
        return 1;
    }

}

$anonymous = new class {
    public function m(): ?int
    {
        return 1;
    }
};

$c1 = function (): ?int {
    return 1;
};
$c2 = function (): \Generator|int {
    yield 1;
};
$c3 = function (): ?int {
    return;
};
$c4 = function (): ?int {
    return null;
};
$c5 = function () {
    return 1;
};
$c6 = function (): int {
    return 1;
};
$a1 = fn (): ?int => 1;
$a2 = fn (): \Generator|int => yield 1;
$a3 = fn (): ?int => null;
$a4 = fn () => 1;
$a5 = fn (): int => 1;
