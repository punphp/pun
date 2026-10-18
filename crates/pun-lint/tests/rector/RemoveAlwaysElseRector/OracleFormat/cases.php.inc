<?php

function colon($a)
{
    if ($a):
        return 1;
    else:
        echo 2;
    endif;
}

function bare($a)
{
    if ($a) return 1; else echo 2;
}

function viaElseIf($a)
{
    if ($a) {
        return 1;
    } else if ($a > 2) {
        return 2;
    } else {
        return 3;
    }
}

function commented($a)
{
    if ($a) {
        return 1;
    }
    // before else
    else {
        echo 2;
        // trailing
    }
}

function nested($a)
{
    foreach ($a as $b)
        if ($b) {
            continue;
        } else {
            echo $b;
        }
}

function breakNotLeaving($a)
{
    foreach ($a as $b) {
        if ($b) {
            break;
        } else {
            echo $b;
        }
    }
}
-----
<?php

function colon($a)
{
    if ($a):
        return 1;
    endif;
    echo 2;
}

function bare($a)
{
    if ($a) return 1;
    echo 2;
}

function viaElseIf($a)
{
    if ($a) {
        return 1;
    }
    if ($a > 2) {
        return 2;
    }
    return 3;
}

function commented($a)
{
    if ($a) {
        return 1;
    }
    echo 2;
    // trailing

}

function nested($a)
{
    foreach ($a as $b) {
        if ($b) {
            continue;
        }
        echo $b;
    }
}

function breakNotLeaving($a)
{
    foreach ($a as $b) {
        if ($b) {
            break;
        } else {
            echo $b;
        }
    }
}
