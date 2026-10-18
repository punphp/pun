<?php

function replaced($a)
{
    switch ($a) {
        case 'a':
            return 1;
        // b
        case 'b':
            return 2;

        case 'c':
            return 1;
        /* d */
        case 'd':
            return 3;
        // e
        case 'e':
            return 2;
    }
}

function backwards($a)
{
    switch ($a) {
        case 1:
            echo 'x';
            break;
        case 2:
            echo 'y';
            break;
        // three
        case 3:
            echo 'z';
            break;
        case 4:
            echo 'y';
            break;

        case 5:
            echo 'x';
            break;
    }
}
-----
<?php

function replaced($a)
{
    switch ($a) {
        case 'a':

        case 'c':
            return 1;
        // b
        case 'b':
        // e
        case 'e':
            return 2;
        /* d */
        case 'd':
            return 3;
    }
}

function backwards($a)
{
    switch ($a) {
        case 1:
        case 5:
            echo 'x';
            break;
        case 2:
        case 4:
            echo 'y';
            break;

        // three
        case 3:
            echo 'z';
            break;
    }
}
