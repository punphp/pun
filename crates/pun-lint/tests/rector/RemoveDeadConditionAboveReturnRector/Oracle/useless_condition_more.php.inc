<?php

function a($a)
{
    foo();
    return 1;
}

function b($a)
{
    if ($a) {
    }
    return 1;
}

class A
{
    public int $value {
        get {
            if ($this->a) {
            }
            return 1;
        }
    }
}
-----
<?php

function a($a)
{
    foo();
    return 1;
}

function b($a)
{
    return 1;
}

class A
{
    public int $value {
        get {
            if ($this->a) {
            }
            return 1;
        }
    }
}
