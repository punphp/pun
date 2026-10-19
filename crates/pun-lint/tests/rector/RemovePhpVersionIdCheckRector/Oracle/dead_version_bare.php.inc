<?php

function bare($a)
{
    if ($a)
        if (PHP_VERSION_ID > 50000) {
            echo 1;
            echo 2;
        }
}

function sole()
{
    if (PHP_VERSION_ID > 50000) {
        echo 1;
        echo 2;
    }
}

function single($a)
{
    if ($a)
        if (PHP_VERSION_ID > 50000) {
            echo 1;
        }
    else
        echo 3;
}
-----
<?php

function bare($a)
{
    if ($a) {
        echo 1;
        echo 2;
    }
}

function sole()
{
    echo 1;
    echo 2;
}

function single($a)
{
    if ($a)
        echo 1;
}
