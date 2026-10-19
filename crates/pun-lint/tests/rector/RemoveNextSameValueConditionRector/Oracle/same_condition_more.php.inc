<?php

function a($a)
{
    if ($a) {
        foo();
    }
    foo();
}

class A
{
    public int $value {
        get {
            if ($this->a) {
                foo();
            }
            if ($this->a) {
                bar();
            }
            return 1;
        }
    }
}
