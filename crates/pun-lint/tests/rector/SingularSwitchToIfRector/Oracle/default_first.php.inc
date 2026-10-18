<?php

function a($value)
{
    switch ($value) {
        default:
            echo 1;
            echo 2;
    }

    // after
    return 1;
}
-----
<?php

function a($value)
{
    echo 1;
    echo 2;
    // after
    return 1;
}
