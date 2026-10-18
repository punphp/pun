<?php

class Hook
{
    public int $a {
        get {
            $var = 0;
            if ($this->b) {
                $var = 1;
            }
            return $var;
        }
    }
}

function noIfsAfterAssign($a)
{
    if ($a) {
        foo();
    }
    $var = 1;
    return $var;
}

function notFollowedByReturn($a)
{
    $var = 0;
    if ($a) {
        $var = 1;
    }
    foo();
    return $var;
}

function otherReturned($a)
{
    $var = 0;
    if ($a) {
        $var = 1;
    }
    return $other;
}
