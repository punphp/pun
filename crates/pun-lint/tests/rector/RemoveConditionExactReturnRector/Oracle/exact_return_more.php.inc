<?php

function a($a)
{
    if ($a === 1) {
        foo();
        return 1;
    }
    if ($a) {
        return 1;
    }
    if ($a > 1) {
        return 1;
    }
    if ($a === 1) {
        return 2;
    }
    if ($a === 1) {
        return 1;
    }
    foo();
    if ($a === foo()) {
        return foo();
    }
    return $a;
}

class A
{
    public int $value {
        get {
            if ($this->a === 1) {
                return 1;
            }
            return $this->a;
        }
    }
}
