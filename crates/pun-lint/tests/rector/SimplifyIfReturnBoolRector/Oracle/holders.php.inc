<?php

function oracle_holders($a, $b, $x)
{
    if ($a)   {
        if ($b) {
            return false;
        }
        return true;
    }  elseif ($b)  {
        if ($a) {
            return true;
        }
        return false;
    }   else {
        if ($x) {
            return true;
        }
        return false;
    }

    if ($b) {
        echo 1;
    } elseif ($a) {
        if ($x) {
            return true;
        }
        return false;
    } else if ($x) {
        if ($a) {
            return true;
        }
        return false;
    }

    switch ($x) {
        case 1 :
            if ($a) {
                return true;
            }
            return false;
        default:
            if ($b) {
                return true;
            }
            return false;
    }
}
-----
<?php

function oracle_holders($a, $b, $x)
{
    if ($a) {
        return !$b;
    } elseif ($b) {
        return (bool) $a;
    } else {
        return (bool) $x;
    }

    if ($b) {
        echo 1;
    } elseif ($a) {
        return (bool) $x;
    } else if ($x) {
        return (bool) $a;
    }

    switch ($x) {
        case 1:
            return (bool) $a;
        default:
            return (bool) $b;
    }
}
