<?php

function a($value)
{
    $x = 1;

    // gone
    switch ($value) {
        default:
            // kept
            foo(
                1
            );
    }
}
-----
<?php

function a($value)
{
    $x = 1;

    // kept
    foo(
        1
    );
}
