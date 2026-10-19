<?php

function f($a)
{
    if ($a > 1) {
        echo 1;
    }
    if ($a > 2) {
        echo 2;
    }
    if ($a > 3) {
        echo 3;
    }
    if ($a > 3) {
    }
}
-----
<?php

function f($a)
{
    if ($a > 1) {
        echo 1;
    }
    if ($a > 2) {
        echo 2;
    }
    if ($a > 3) {
        echo 3;
    }
}
