<?php

function a()
{
    if (PHP_VERSION_ID < 80000) {
        return 1;
    }
    if (PHP_VERSION_ID >= 70400) {
        echo 'a';
        echo 'b';
    }
    if (PHP_VERSION_ID < 70000) {
        echo 'c';
    } else {
        echo 'd';
    }
    if (\PHP_VERSION_ID > 80500) {
        echo 'e';
    }
    if (PHP_VERSION_ID >= 80000) echo 'f';
    // keep
    if (70100 <= PHP_VERSION_ID) {
        echo 'g';
    }
}
-----
<?php

function a()
{
    echo 'a';
    echo 'b';
    if (PHP_VERSION_ID < 70000) {
        echo 'c';
    } else {
        echo 'd';
    }
    if (\PHP_VERSION_ID > 80500) {
        echo 'e';
    }
    echo 'f';
    // keep
    if (70100 <= PHP_VERSION_ID) {
        echo 'g';
    }
}
