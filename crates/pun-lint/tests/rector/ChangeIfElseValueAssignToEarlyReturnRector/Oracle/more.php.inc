<?php

class Hook
{
    public int $a {
        get {
            if ($this->b) {
                $x = 1;
            } else {
                $x = 2;
            }
            return $x;
        }
    }
}

function noValue($a)
{
    if ($a) {
        $x = 1;
    } else {
        $x = 2;
    }
    return;
}

function notIf($a)
{
    foo();
    return $a;
}

function withElseIf($a)
{
    if ($a) {
        $x = 1;
    } elseif ($a > 1) {
        $x = 3;
    } else {
        $x = 2;
    }
    return $x;
}

function twoReturns($a)
{
    foo();
    return $a;
    if ($a) {
        $x = 1;
    } else {
        $x = 2;
    }
    return $x;
}
-----
<?php

class Hook
{
    public int $a {
        get {
            if ($this->b) {
                $x = 1;
            } else {
                $x = 2;
            }
            return $x;
        }
    }
}

function noValue($a)
{
    if ($a) {
        $x = 1;
    } else {
        $x = 2;
    }
    return;
}

function notIf($a)
{
    foo();
    return $a;
}

function withElseIf($a)
{
    if ($a) {
        $x = 1;
    } elseif ($a > 1) {
        $x = 3;
    } else {
        $x = 2;
    }
    return $x;
}

function twoReturns($a)
{
    foo();
    return $a;
    if ($a) {
        return 1;
    }
    return 2;
}
