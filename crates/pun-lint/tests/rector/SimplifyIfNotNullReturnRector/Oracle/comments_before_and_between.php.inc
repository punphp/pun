<?php

function a($x)
{
    // before if
    if ($x !== null) {
        // inside
        return $x;
    }
    // between
    return null;
}
-----
<?php

function a($x)
{
    // inside
    return $x;
}
