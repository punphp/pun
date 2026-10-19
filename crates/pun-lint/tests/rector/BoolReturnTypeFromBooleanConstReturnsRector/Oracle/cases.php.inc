<?php

namespace App;

function yes()
{
    return true;
}

function either($a)
{
    if ($a) {
        return TRUE;
    }

    return false;
}

function maybe($a)
{
    if ($a) {
        return true;
    }
}

function withNull($a)
{
    if ($a) {
        return null;
    }

    return false;
}

function viaIfElse($a)
{
    if ($a) {
        return true;
    } elseif ($a > 1) {
        return false;
    } else {
        return true;
    }
}

function viaDoWhile($a)
{
    do {
        return false;
    } while ($a);
}

function viaBreakLoop($a)
{
    while (true) {
        if ($a) {
            break;
        }

        return true;
    }
}

final class Checker
{
    public function check()
    {
        return false;
    }

    protected function inner()
    {
        $c = new class {
            public function x()
            {
                return 1;
            }
        };

        return true;
    }
}

class Child extends \ArrayIterator
{
    public function valid()
    {
        return true;
    }
}
-----
<?php

namespace App;

function yes(): bool
{
    return true;
}

function either($a): bool
{
    if ($a) {
        return TRUE;
    }

    return false;
}

function maybe($a)
{
    if ($a) {
        return true;
    }
}

function withNull($a)
{
    if ($a) {
        return null;
    }

    return false;
}

function viaIfElse($a): bool
{
    if ($a) {
        return true;
    } elseif ($a > 1) {
        return false;
    } else {
        return true;
    }
}

function viaDoWhile($a): bool
{
    do {
        return false;
    } while ($a);
}

function viaBreakLoop($a)
{
    while (true) {
        if ($a) {
            break;
        }

        return true;
    }
}

final class Checker
{
    public function check(): bool
    {
        return false;
    }

    protected function inner(): bool
    {
        $c = new class {
            public function x()
            {
                return 1;
            }
        };

        return true;
    }
}

class Child extends \ArrayIterator
{
    public function valid()
    {
        return true;
    }
}
