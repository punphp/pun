<?php

function trailingComment()
{
    return 1; // done
    $a = 2;
    // after
}

function onlyComment()
{
    throw new \Exception();
    // keep
}

function exits($a)
{
    if ($a) {
        exit(1);
        echo 'no';
    }
    die('x');
    echo 'never';
}
-----
<?php

function trailingComment()
{
    return 1;
}

function onlyComment()
{
    throw new \Exception();
    // keep
}

function exits($a)
{
    if ($a) {
        exit(1);
    }
    die('x');
}
