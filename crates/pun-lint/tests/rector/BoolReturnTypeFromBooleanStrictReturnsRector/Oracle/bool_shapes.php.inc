<?php

namespace App;

function is_x(): bool
{
    return true;
}

function paren($a)
{
    return ($a === 1);
}

function dynamic($f)
{
    if ($f) {
        return $f();
    }
    return $f === 1;
}

function namespaced($a)
{
    if ($a) {
        return is_x();
    }
    return $a === 1;
}

function qualified($a)
{
    if ($a) {
        return \is_int($a);
    }
    return $a === 1;
}
-----
<?php

namespace App;

function is_x(): bool
{
    return true;
}

function paren($a): bool
{
    return ($a === 1);
}

function dynamic($f)
{
    if ($f) {
        return $f();
    }
    return $f === 1;
}

function namespaced($a)
{
    if ($a) {
        return is_x();
    }
    return $a === 1;
}

function qualified($a): bool
{
    if ($a) {
        return \is_int($a);
    }
    return $a === 1;
}
