<?php

function c($x)
{
    if ($x !== null) {
        return $x;
    }

    return null;

    echo 1;
}
-----
<?php

function c($x)
{
    return $x;
    echo 1;
}
