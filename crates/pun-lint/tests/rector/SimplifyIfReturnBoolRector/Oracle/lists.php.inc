<?php

function oracle_return_bool_colon($a)
{
    if ($a):
        return true;
    endif;
    return false;
}

function oracle_return_bool_nested($a, $b)
{
    if ($a) {
        if ($b) {
            return false;
        }

        return true;
    }

    return false;
}

$closure = function ($a) {
    if ($a === 1) {
        return false;
    }
    return true;
};

switch ($x) {
    case 1:
        if ($x === 2) {
            return true;
        }
        return false;
}
-----
<?php

function oracle_return_bool_colon($a)
{
    return (bool) $a;
}

function oracle_return_bool_nested($a, $b)
{
    if ($a) {
        return !$b;
    }

    return false;
}

$closure = function ($a) {
    return $a !== 1;
};

switch ($x) {
    case 1:
        return $x === 2;
}
