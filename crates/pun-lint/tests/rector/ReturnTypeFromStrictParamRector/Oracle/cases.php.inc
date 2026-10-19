<?php

namespace App;

use Countable;

function a(? int $a)
{
    return $a;
}

function b(int|string $a)
{
    return $a;
}

function c(\Countable $a, $b)
{
    if ($b) {
        return $a;
    }

    return $a;
}

function d(Countable $a)
{
    $f = function () use ($a) {
        return 1;
    };

    return $a;
}

function e(int $a)
{
    $a++;

    return $a;
}

function f(array $a)
{
    $a[] = 1;

    return $a;
}

function h(int $a)
{
    foreach ([1] as $a) {
    }

    return $a;
}

function i(int $a)
{
    return $a;

    return 1;
}

function j(int $a, ?string $s)
{
    if ($a) {
        return null;
    }

    return $s;
}

function g1(int $a, int $b, $c)
{
    if ($c) {
        return $b;
    }

    return $a;
}

function g2(int $a, string $b, $c)
{
    if ($c) {
        return $b;
    }

    return $a;
}

function g3(int $a, int $b)
{
    if ($b > 0) {
        return $b;
    }

    return $a;
}

function g4(int $a, $c)
{
    if ($c) {
        return 5;
    }

    return $a;
}

function g5(string $a, string $b)
{
    if ($b) {
        return $b;
    }

    return $a;
}
-----
<?php

namespace App;

use Countable;

function a(? int $a): ? int
{
    return $a;
}

function b(int|string $a)
{
    return $a;
}

function c(\Countable $a, $b): \Countable
{
    if ($b) {
        return $a;
    }

    return $a;
}

function d(Countable $a): Countable
{
    $f = function () use ($a) {
        return 1;
    };

    return $a;
}

function e(int $a): int
{
    $a++;

    return $a;
}

function f(array $a): array
{
    $a[] = 1;

    return $a;
}

function h(int $a): int
{
    foreach ([1] as $a) {
    }

    return $a;
}

function i(int $a)
{
    return $a;

    return 1;
}

function j(int $a, ?string $s)
{
    if ($a) {
        return null;
    }

    return $s;
}

function g1(int $a, int $b, $c): int
{
    if ($c) {
        return $b;
    }

    return $a;
}

function g2(int $a, string $b, $c)
{
    if ($c) {
        return $b;
    }

    return $a;
}

function g3(int $a, int $b)
{
    if ($b > 0) {
        return $b;
    }

    return $a;
}

function g4(int $a, $c)
{
    if ($c) {
        return 5;
    }

    return $a;
}

function g5(string $a, string $b): string
{
    if ($b) {
        return $b;
    }

    return $a;
}
