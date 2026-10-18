<?php

function f($a)
{
    switch ($a) {
        case 1:
            return 'x';

        // before two
        case 2:
            return 'y';

        case 3:
            return 'x';
        default:
            return 'y';
    }
}

function colon($a)
{
    switch ($a):
        case 1:
            echo 1;
            break;
        case 2:
            echo 2;
            break;
        case 3:
            echo 1;
            break;
    endswitch;
}

function breaks($a)
{
    switch ($a) {
        case 1:
            break;
        case 2:
            break;
    }
}

function noBreak($a)
{
    switch ($a) {
        case 1:
            echo 1;
        case 2:
            echo 1;
    }
}
-----
<?php

function f($a)
{
    switch ($a) {
        case 1:

        case 3:
            return 'x';
        // before two
        case 2:
        default:
            return 'y';
    }
}

function colon($a)
{
    switch ($a):
        case 1:
        case 3:
            echo 1;
            break;
        case 2:
            echo 2;
            break;
    endswitch;
}

function breaks($a)
{
    switch ($a) {
        case 1:
        case 2:
            break;
    }
}

function noBreak($a)
{
    switch ($a) {
        case 1:
            echo 1;
        case 2:
            echo 1;
    }
}
