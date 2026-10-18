<?php

function a($x)
{
    $y = 1;
    if ($x !== null) {
        $z = 2;
        return $x;
    }
    return null;
}

function b($x)
{
    if ($x) {
        if ($x !== null) {
            return $x;
        }
        return null;
    }
    return 1;
}

final class C
{
    public ?int $p {
        get {
            if ($this->p !== null) {
                return $this->p;
            }
            return null;
        }
    }
}

function d($x)
{
    $y = 1;
    if ($x !== null) {
        echo $x;
    }
    return null;
}
-----
<?php

function a($x)
{
    $y = 1;
    if ($x !== null) {
        $z = 2;
        return $x;
    }
    return null;
}

function b($x)
{
    if ($x) {
        return $x;
    }
    return 1;
}

final class C
{
    public ?int $p {
        get {
            if ($this->p !== null) {
                return $this->p;
            }
            return null;
        }
    }
}

function d($x)
{
    $y = 1;
    if ($x !== null) {
        echo $x;
    }
    return null;
}
