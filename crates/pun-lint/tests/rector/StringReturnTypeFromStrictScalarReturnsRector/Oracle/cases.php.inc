<?php

namespace App;

function name()
{
    return 'a';
}

function names($a)
{
    if ($a) {
        return "b {$a}";
    }

    return <<<TXT
    c
    TXT;
}

function shell()
{
    return `ls`;
}

function concat($a)
{
    return 'a' . $a;
}

function number($a)
{
    if ($a) {
        return -1;
    }

    return 0x10;
}

function plus()
{
    return +1;
}

function decimal($a)
{
    if ($a) {
        return -1.5;
    }

    return 2e3;
}

function mixedNumbers($a)
{
    if ($a) {
        return 1;
    }

    return 1.0;
}

function nothing()
{
}

function maybe($a)
{
    if ($a) {
        return 1;
    }
}

final class Values
{
    public function label()
    {
        return 'x';
    }

    public function __toString()
    {
        return 'x';
    }
}
-----
<?php

namespace App;

function name(): string
{
    return 'a';
}

function names($a): string
{
    if ($a) {
        return "b {$a}";
    }

    return <<<TXT
    c
    TXT;
}

function shell()
{
    return `ls`;
}

function concat($a)
{
    return 'a' . $a;
}

function number($a)
{
    if ($a) {
        return -1;
    }

    return 0x10;
}

function plus()
{
    return +1;
}

function decimal($a)
{
    if ($a) {
        return -1.5;
    }

    return 2e3;
}

function mixedNumbers($a)
{
    if ($a) {
        return 1;
    }

    return 1.0;
}

function nothing()
{
}

function maybe($a)
{
    if ($a) {
        return 1;
    }
}

final class Values
{
    public function label(): string
    {
        return 'x';
    }

    public function __toString()
    {
        return 'x';
    }
}
