<?php

function f0(array $items, $x, $y)
{
    foo();
    foreach ($items as $k => $v) {
        if ($v === $x) {
            return true;
        }
    }
    return false;
}

function f1(array $items, $x, $y)
{
    foo();
    foreach ($items as $v) {
        echo $v;
    }
    return false;
}

function f2(array $items, $x, $y)
{
    foo();
    foreach ($items as $v) {
        if ($v > $x) {
            return true;
        }
    }
    return false;
}

function f3(array $items, $x, $y)
{
    foo();
    foreach ($items as $v) {
        if ($y === $x) {
            return true;
        }
    }
    return false;
}

function f4(array $items, $x, $y)
{
    foo();
    foreach ($items as $v) {
        if ($v === $v + 1) {
            return true;
        }
    }
    return false;
}

function f5(array $items, $x, $y)
{
    foo();
    foreach ($items as $v) {
        if ($v === $x) {
            return;
        }
    }
    return false;
}

function f6(array $items, $x, $y)
{
    foo();
    foreach ($items as $v) {
        if ($x == $v) {
            return false;
        }
    }
    return true;
}

class A
{
    public $value {
        get {
            foreach ([1, 2] as $v) {
                if ($v === 1) {
                    return true;
                }
            }
            return false;
        }
    }
}
-----
<?php

function f0(array $items, $x, $y)
{
    foo();
    foreach ($items as $k => $v) {
        if ($v === $x) {
            return true;
        }
    }
    return false;
}

function f1(array $items, $x, $y)
{
    foo();
    foreach ($items as $v) {
        echo $v;
    }
    return false;
}

function f2(array $items, $x, $y)
{
    foo();
    foreach ($items as $v) {
        if ($v > $x) {
            return true;
        }
    }
    return false;
}

function f3(array $items, $x, $y)
{
    foo();
    foreach ($items as $v) {
        if ($y === $x) {
            return true;
        }
    }
    return false;
}

function f4(array $items, $x, $y)
{
    foo();
    foreach ($items as $v) {
        if ($v === $v + 1) {
            return true;
        }
    }
    return false;
}

function f5(array $items, $x, $y)
{
    foo();
    foreach ($items as $v) {
        if ($v === $x) {
            return;
        }
    }
    return false;
}

function f6(array $items, $x, $y)
{
    foo();
    return !in_array($x, $items);
}

class A
{
    public $value {
        get {
            foreach ([1, 2] as $v) {
                if ($v === 1) {
                    return true;
                }
            }
            return false;
        }
    }
}
