<?php

namespace App;

class Thing
{
}

function destructured($x)
{
    $thing = new Thing();
    if ($x) {
        [$other] = [$thing];
    }

    return $thing;
}

function looped($x)
{
    $thing = new Thing();
    foreach ($x as $y) {
    }

    return $thing;
}

function incremented($x)
{
    $thing = new Thing();
    $x++;
    ++$x;

    return $thing;
}

function declared()
{
    $thing = new Thing();
    global $g;
    static $s;

    return $thing;
}

function dynamic($cls)
{
    $other = new $cls();
    $thing = new Thing();

    return $thing;
}

class Maker
{
    public function make($x)
    {
        if ($x) {
            return new static();
        }

        return new Thing();
    }
}
-----
<?php

namespace App;

class Thing
{
}

function destructured($x): \App\Thing
{
    $thing = new Thing();
    if ($x) {
        [$other] = [$thing];
    }

    return $thing;
}

function looped($x): \App\Thing
{
    $thing = new Thing();
    foreach ($x as $y) {
    }

    return $thing;
}

function incremented($x): \App\Thing
{
    $thing = new Thing();
    $x++;
    ++$x;

    return $thing;
}

function declared(): \App\Thing
{
    $thing = new Thing();
    global $g;
    static $s;

    return $thing;
}

function dynamic($cls): \App\Thing
{
    $other = new $cls();
    $thing = new Thing();

    return $thing;
}

class Maker
{
    public function make($x): static|\App\Thing
    {
        if ($x) {
            return new static();
        }

        return new Thing();
    }
}
