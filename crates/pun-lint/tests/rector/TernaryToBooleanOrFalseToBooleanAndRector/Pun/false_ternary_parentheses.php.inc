<?php

function f8(bool $b, bool $c, $d)
{
    return ($b = $d) ? $c : false;
}

function f10(bool $b, bool $c, $d)
{
    return (fn () => 1) ? $c : false;
}
-----
<?php

function f8(bool $b, bool $c, $d)
{
    return ($b = $d) && $c;
}

function f10(bool $b, bool $c, $d)
{
    return (fn () => 1) && $c;
}
