<?php

function f0(bool $b, bool $c, $d)
{
    return $d ?: false;
}

function f1(bool $b, bool $c, $d)
{
    return $d ? $b && $c : false;
}

function f2(bool $b, bool $c, $d)
{
    return $d && $b ? $c : false;
}

function f3(bool $b, bool $c, $d)
{
    return $d ? $b || $c : false;
}

function f4(bool $b, bool $c, $d)
{
    return $d ? $b and $c : false;
}

function f5(bool $b, bool $c, $d)
{
    return $d ? $b xor $c : false;
}

function f6(bool $b, bool $c, $d)
{
    return $d ?? $b ? $c : false;
}

function f7(bool $b, bool $c, $d)
{
    return ($d ? $b : $c) ? $c : false;
}

function f9(bool $b, bool $c, $d)
{
    return (print $d) ? $c : false;
}

function f11(bool $b, bool $c, $d)
{
    return $d ? !$b : false;
}
-----
<?php

function f0(bool $b, bool $c, $d)
{
    return $d ?: false;
}

function f1(bool $b, bool $c, $d)
{
    return $d && ($b && $c);
}

function f2(bool $b, bool $c, $d)
{
    return $d && $b && $c;
}

function f3(bool $b, bool $c, $d)
{
    return $d && ($b || $c);
}

function f4(bool $b, bool $c, $d)
{
    return $d && ($b and $c);
}

function f5(bool $b, bool $c, $d)
{
    return $d && ($b xor $c);
}

function f6(bool $b, bool $c, $d)
{
    return ($d ?? $b) && $c;
}

function f7(bool $b, bool $c, $d)
{
    return ($d ? $b : $c) && $c;
}

function f9(bool $b, bool $c, $d)
{
    return (print $d) && $c;
}

function f11(bool $b, bool $c, $d)
{
    return $d && !$b;
}
