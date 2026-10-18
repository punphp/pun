<?php

function a($value)
{
    $x = 1;

    // comment
    switch ($value) {
        default:
            // one
            echo 1;
            break 2;
            /** two */
            echo 2;
    }

    return $x;
}
-----
<?php

function a($value)
{
    $x = 1;
    // one
    echo 1;
    /** two */
    echo 2;

    return $x;
}
