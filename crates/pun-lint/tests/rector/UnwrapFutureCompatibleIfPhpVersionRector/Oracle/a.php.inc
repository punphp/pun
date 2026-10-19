<?php

function a()
{
    // first
    if (version_compare(PHP_VERSION, '8.0', '>=')) {
        echo 'a';
        echo 'b';
    }
    if (version_compare(PHP_VERSION, '7.0', '<')) {
        echo 'c';
    }
    if (version_compare(PHP_VERSION, '7.0') === -1) {
        echo 'd';
    } else {
        // in else
        echo 'e';
    }
    if (version_compare(PHP_VERSION, '9.0', '<')) {
        echo 'f';
    }
    if (version_compare(PHP_VERSION, 8.1, 'ge')) {
        echo 'g';
    }
}
-----
<?php

function a()
{
    // first
    echo 'a';
    echo 'b';
    // in else
    echo 'e';
    if (version_compare(PHP_VERSION, '9.0', '<')) {
        echo 'f';
    }
    echo 'g';
}
