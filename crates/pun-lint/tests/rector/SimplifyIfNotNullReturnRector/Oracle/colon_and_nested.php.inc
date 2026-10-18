<?php

function m($x)
{
    $a = 1;
    if ($x !== null):
        return $x;
    endif;
    return null;
}

function n($x)
{
    foreach ($x as $y) {
        $z = 1;
        if (\NULL !== $y) {
            return $y;
        }
        return null;
    }
}
-----
<?php

function m($x)
{
    $a = 1;
    return $x;
}

function n($x)
{
    foreach ($x as $y) {
        $z = 1;
        return $y;
    }
}
