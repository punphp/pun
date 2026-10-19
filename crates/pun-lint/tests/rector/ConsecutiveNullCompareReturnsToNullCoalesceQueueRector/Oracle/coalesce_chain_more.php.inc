<?php

function a($a, $b)
{
    if ($a !== null) {
        return;
    }
    if ($a) {
        return $a;
    }
    if ($a != null) {
        return $a;
    }
    if ($a !== null) {
        return $b;
    }
    if ($a !== false) {
        return $a;
    }
    return null;
}

class A
{
    public $value {
        get {
            if ($this->a !== null) {
                return $this->a;
            }
            if ($this->b !== null) {
                return $this->b;
            }
            return null;
        }
    }
}

function b($a, $b)
{
    if ($a !== null) {
        return $a;
    }
    if ($b !== null) {
        return $b;
    }
    return null;
    echo 1;
}
