<?php

function c1($x)
{
    $y = 1;
    if ($x !== null) {
        /*
         * star lines
         */
        return $x;
    }
    return null;
}

function c2($x)
{
    $y = 1;
    if ($x !== null) {
        /*
            text
        */
        return $x;
    }
    return null;
}

function c3($x)
{
    $y = 1;
    if ($x !== null) {
        /* first
                      second */
        return $x;
    }
    return null;
}

function c4($x)
{
    $y = 1;
    if ($x !== null) {
        /*  first
   short */
        return $x;
    }
    return null;
}
-----
<?php

function c1($x)
{
    $y = 1;
    /*
     * star lines
     */
    return $x;
}

function c2($x)
{
    $y = 1;
    /*
        text
    */
    return $x;
}

function c3($x)
{
    $y = 1;
    /* first
       second */
    return $x;
}

function c4($x)
{
    $y = 1;
    /*  first
       short */
    return $x;
}
