<?php

namespace App;

function noop(): void
{
}

function boolParam(bool $b)
{
    return $b;
}

function bareReturn(int $x)
{
    if ($x > 1) {
        return $x;
    }

    return;
}

function onlyBareReturn(int $x)
{
    return;
}

function voidCall(int $x)
{
    if ($x > 1) {
        return $x;
    }

    return noop();
}
-----
<?php

namespace App;

function noop(): void
{
}

function boolParam(bool $b): bool
{
    return $b;
}

function bareReturn(int $x)
{
    if ($x > 1) {
        return $x;
    }

    return;
}

function onlyBareReturn(int $x)
{
    return;
}

function voidCall(int $x)
{
    if ($x > 1) {
        return $x;
    }

    return noop();
}
