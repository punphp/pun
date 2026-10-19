<?php

class A
{
    public int $value {
        get {
            $a = 1;
            $a = 2;
            return $a;
        }
    }
}

function f()
{
    $a = 1;
    $a = 2;
    return $a;
}
-----
<?php

class A
{
    public int $value {
        get {
            $a = 1;
            $a = 2;
            return $a;
        }
    }
}

function f()
{
    $a = 2;
    return $a;
}
