<?php

class Hooks
{
    public int $a {
        get {
            return 1;
            echo 'hook';
        }
    }
}

function emptyStatement()
{
    return 1; ; // trailing
}

function colonSwitch($a)
{
    switch ($a):
        case 1:
            return 1;
        default:
            throw new Exception();
    endswitch;
    echo 'after colon switch';
}

function nestedJump($a)
{
    switch ($a) {
        case 1:
            $f = function () {
                foreach ([] as $x) {
                    break;
                }
            };
            return $f;
        default:
            return 2;
    }
    echo 'after switch with closure';
}

function finallyReturns()
{
    try {
        foo();
    } finally {
        return 1;
    }
    echo 'after finally';
}

function catchGoesOn()
{
    try {
        return 1;
    } catch (Exception $e) {
        echo 1;
    }
    echo 'reached';
}

function elseIfGoesOn($a, $b)
{
    if ($a) {
        return 1;
    } elseif ($b) {
        echo 1;
    } else {
        return 2;
    }
    echo 'reached';
}

function colonIf($a, $b)
{
    if ($a):
        return 1;
    elseif ($b):
        return 2;
    else:
        return 3;
    endif;
    echo 'after colon if';
}

function colonBodies($a)
{
    if ($a):
        return 1;
        echo 'if';
    elseif ($a > 1):
        return 2;
        echo 'elseif';
    else:
        return 3;
        echo 'else';
    endif;
    while ($a):
        break;
        echo 'while';
    endwhile;
    for (;;):
        continue;
        echo 'for';
    endfor;
    foreach ($a as $b):
        return $b;
        echo 'foreach';
    endforeach;
    declare(ticks=1):
        return 4;
        echo 'declare';
    enddeclare;
}

$anon = new class {
    public function f()
    {
        return 1;
        echo 'anonymous';
    }
};
-----
<?php

class Hooks
{
    public int $a {
        get {
            return 1;
            echo 'hook';
        }
    }
}

function emptyStatement()
{
    return 1; ; // trailing
}

function colonSwitch($a)
{
    switch ($a):
        case 1:
            return 1;
        default:
            throw new Exception();
    endswitch;
}

function nestedJump($a)
{
    switch ($a) {
        case 1:
            $f = function () {
                foreach ([] as $x) {
                    break;
                }
            };
            return $f;
        default:
            return 2;
    }
}

function finallyReturns()
{
    try {
        foo();
    } finally {
        return 1;
    }
}

function catchGoesOn()
{
    try {
        return 1;
    } catch (Exception $e) {
        echo 1;
    }
    echo 'reached';
}

function elseIfGoesOn($a, $b)
{
    if ($a) {
        return 1;
    } elseif ($b) {
        echo 1;
    } else {
        return 2;
    }
    echo 'reached';
}

function colonIf($a, $b)
{
    if ($a):
        return 1;
    elseif ($b):
        return 2;
    else:
        return 3;
    endif;
}

function colonBodies($a)
{
    if ($a):
        return 1;
    elseif ($a > 1):
        return 2;
    else:
        return 3;
    endif;
}

$anon = new class {
    public function f()
    {
        return 1;
    }
};
