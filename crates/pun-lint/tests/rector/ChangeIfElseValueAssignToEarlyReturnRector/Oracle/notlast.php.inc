<?php

function notLastPlain($a)
{
    if ($a) {
        $x = 1;
    } else {
        echo 'else';
        $x = 2;
    }
    return $x;
    echo 'unreachable';
}
-----
<?php

function notLastPlain($a)
{
    if ($a) {
        return 1;
    }
    return 2;
    echo 'else';
    echo 'unreachable';
}
