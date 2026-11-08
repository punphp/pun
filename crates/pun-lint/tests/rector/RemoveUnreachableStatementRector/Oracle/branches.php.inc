<?php

function ifTrailingComment($a)
{
    if ($a) {
        return 1;
        // a comment ends this branch
    } else {
        return 2;
    }
    echo 'reached?';
}

function viaElseIf($a)
{
    if ($a) {
        return 1;
    } else if ($a > 1) {
        return 2;
    } else {
        return 3;
    }
    echo 'else if is an else holding an if';
}

function colon($a)
{
    while ($a):
        break;
        echo 'after break';
    endwhile;
    if ($a):
        return 1;
    else:
        return 2;
    endif;
    echo 'never';
}

function tryCatch()
{
    try {
        return 1;
    } catch (\Exception $e) {
        throw $e;
    }
    echo 'never';
}

function switchContinue($a)
{
    foreach ([1] as $x) {
        switch ($a) {
            case 1:
                return 1;
            default:
                continue 2;
        }
        echo 'reached by nothing but kept';
    }
}

function gotoJump()
{
    goto end;
    echo 'skipped';
    end:
    echo 'end';
}
-----
<?php

function ifTrailingComment($a)
{
    if ($a) {
        return 1;
        // a comment ends this branch
    } else {
        return 2;
    }
}

function viaElseIf($a)
{
    if ($a) {
        return 1;
    } else if ($a > 1) {
        return 2;
    } else {
        return 3;
    }
    echo 'else if is an else holding an if';
}

function colon($a)
{
    while ($a):
        break;
    endwhile;
    if ($a):
        return 1;
    else:
        return 2;
    endif;
}

function tryCatch()
{
    try {
        return 1;
    } catch (\Exception $e) {
        throw $e;
    }
}

function switchContinue($a)
{
    foreach ([1] as $x) {
        switch ($a) {
            case 1:
                return 1;
            default:
                continue 2;
        }
        echo 'reached by nothing but kept';
    }
}

function gotoJump()
{
    goto end;
}
