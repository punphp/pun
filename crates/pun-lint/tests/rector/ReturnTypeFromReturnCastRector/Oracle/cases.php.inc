<?php

namespace App;

function toInt($a)
{
    return (int) $a;
}

function toIntTwice($a)
{
    if ($a) {
        return (integer) $a;
    }

    return (int) ($a + 1);
}

function mixedCasts($a)
{
    if ($a) {
        return (int) $a;
    }

    return (float) $a;
}

function maybeVoid($a)
{
    if ($a) {
        return (string) $a;
    }
}

function emptyReturn($a)
{
    if ($a) {
        return;
    }

    return (bool) $a;
}

function generator($a)
{
    yield 1;

    return (int) $a;
}

function parenthesized($a)
{
    return ((float) $a);
}

function arrays($a)
{
    return (array) $a;
}

function objects($a)
{
    return (object) $a;
}

function nested($a)
{
    $f = function () {
        return 1;
    };

    return (string) $a;
}

function switching($a)
{
    switch ($a) {
        case 1:
            return (int) $a;
        default:
            return (int) 2 * $a;
    }
}

function alwaysThrows($a)
{
    try {
        return (bool) $a;
    } catch (\Throwable $e) {
        throw $e;
    }
}

function whileTrue($a)
{
    while (true) {
        return (int) $a;
    }
}

final class Service
{
    public function name($a)
    {
        return (string) $a;
    }

    private function count($a)
    {
        return (int) $a;
    }

    public function __toString()
    {
        return (string) 1;
    }
}

abstract class Base
{
    public function value($a)
    {
        return (int) $a;
    }
}
-----
<?php

namespace App;

function toInt($a): int
{
    return (int) $a;
}

function toIntTwice($a): int
{
    if ($a) {
        return (integer) $a;
    }

    return (int) ($a + 1);
}

function mixedCasts($a)
{
    if ($a) {
        return (int) $a;
    }

    return (float) $a;
}

function maybeVoid($a)
{
    if ($a) {
        return (string) $a;
    }
}

function emptyReturn($a)
{
    if ($a) {
        return;
    }

    return (bool) $a;
}

function generator($a)
{
    yield 1;

    return (int) $a;
}

function parenthesized($a): float
{
    return ((float) $a);
}

function arrays($a): array
{
    return (array) $a;
}

function objects($a): \stdClass
{
    return (object) $a;
}

function nested($a): string
{
    $f = function () {
        return 1;
    };

    return (string) $a;
}

function switching($a)
{
    switch ($a) {
        case 1:
            return (int) $a;
        default:
            return (int) 2 * $a;
    }
}

function alwaysThrows($a): bool
{
    try {
        return (bool) $a;
    } catch (\Throwable $e) {
        throw $e;
    }
}

function whileTrue($a): int
{
    while (true) {
        return (int) $a;
    }
}

final class Service
{
    public function name($a): string
    {
        return (string) $a;
    }

    private function count($a): int
    {
        return (int) $a;
    }

    public function __toString()
    {
        return (string) 1;
    }
}

abstract class Base
{
    public function value($a)
    {
        return (int) $a;
    }
}
