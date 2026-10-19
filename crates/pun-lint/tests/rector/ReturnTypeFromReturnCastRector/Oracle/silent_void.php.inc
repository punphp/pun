<?php

namespace Pun\SilentVoid;

function fail(): never
{
    throw new \RuntimeException();
}

function tryFinallyReturns($a)
{
    try {
        if ($a) {
            return (int) $a;
        }
    } finally {
        return (int) $a;
    }
}

function tryAndCatchesReturn($a)
{
    try {
        return (int) $a;
    } catch (\RuntimeException $e) {
        return (int) $a;
    } catch (\LogicException $e) {
        exit(1);
    }
}

function catchFallsThrough($a)
{
    try {
        return (int) $a;
    } catch (\RuntimeException $e) {
        echo 1;
    }
}

function tryFallsThrough($a)
{
    try {
        if ($a) {
            return (int) $a;
        }
    } catch (\RuntimeException $e) {
        return (int) $a;
    }
}

function switchDefault($a)
{
    switch ($a) {
        case 1:
        case 2:
            return (int) $a;
        default:
            return (int) $a;
    }
}

function switchNoDefault($a)
{
    switch ($a) {
        case 1:
            return (int) $a;
    }
}

function switchEmptyDefault($a)
{
    switch ($a) {
        case 1:
            return (int) $a;
        default:
    }
}

function switchCaseFallsOut($a)
{
    switch ($a):
        case 1:
            echo 1;
            break;
        default:
            return (int) $a;
    endswitch;
}

function switchColon($a)
{
    switch ($a):
        case 1:
            return (int) $a;
        default:
            die();
    endswitch;
}

function ifColon($a)
{
    if ($a):
        return (int) $a;
    elseif ($a > 1):
        return (int) $a;
    else:
        return (int) $a;
    endif;
}

function ifColonNoElse($a)
{
    if ($a):
        return (int) $a;
    endif;
}

function ifColonElseIfFalls($a)
{
    if ($a):
        return (int) $a;
    elseif ($a > 1):
        echo 1;
    else:
        return (int) $a;
    endif;
}

function ifElseIfFalls($a)
{
    if ($a) {
        return (int) $a;
    } elseif ($a > 1) {
        echo 1;
    } else {
        return (int) $a;
    }
}

function ifElseFalls($a)
{
    if ($a) {
        return (int) $a;
    } else {
        echo 1;
    }
}

function ifAll($a)
{
    if ($a) {
        return (int) $a;
    } elseif ($a > 1) {
        return (int) $a;
    } else {
        throw new \RuntimeException();
    }
}

function whileTrue($a)
{
    while (true) {
        if ($a) {
            return (int) $a;
        }
    }
}

function whileTrueBreak($a)
{
    while (true) {
        if ($a) {
            return (int) $a;
        }
        break;
    }
}

function whileTrueColon($a)
{
    while (true):
        if ($a) {
            return (int) $a;
        }
    endwhile;
}

function whileTrueContinueInClosure($a)
{
    while (true) {
        $f = function () {
            foreach ([1] as $x) {
                continue;
            }
        };
        if ($a) {
            return (int) $a;
        }
    }
}

function whileCondition($a)
{
    while ($a) {
        return (int) $a;
    }
}

function doReturns($a)
{
    do {
        return (int) $a;
    } while ($a);
}

function doTrue($a)
{
    do {
        if ($a) {
            return (int) $a;
        }
    } while (true);
}

function doTrueGoto($a)
{
    do {
        if ($a) {
            return (int) $a;
        }
        goto end;
    } while (true);
    end:
}

function doFalls($a)
{
    do {
        if ($a) {
            return (int) $a;
        }
    } while ($a);
}

function neverCall($a)
{
    if ($a) {
        return (int) $a;
    }
    fail();
}

function yields($a)
{
    if ($a) {
        return (int) $a;
    }
    yield 1;
}

class Methods
{
    public function stop(): never
    {
        exit;
    }

    public static function stopStatic(): never
    {
        exit;
    }

    public function viaThis($a)
    {
        if ($a) {
            return (int) $a;
        }
        $this->stop();
    }

    public function viaStatic($a)
    {
        if ($a) {
            return (int) $a;
        }
        self::stopStatic();
    }
}
-----
<?php

namespace Pun\SilentVoid;

function fail(): never
{
    throw new \RuntimeException();
}

function tryFinallyReturns($a): int
{
    try {
        if ($a) {
            return (int) $a;
        }
    } finally {
        return (int) $a;
    }
}

function tryAndCatchesReturn($a): int
{
    try {
        return (int) $a;
    } catch (\RuntimeException $e) {
        return (int) $a;
    } catch (\LogicException $e) {
        exit(1);
    }
}

function catchFallsThrough($a)
{
    try {
        return (int) $a;
    } catch (\RuntimeException $e) {
        echo 1;
    }
}

function tryFallsThrough($a)
{
    try {
        if ($a) {
            return (int) $a;
        }
    } catch (\RuntimeException $e) {
        return (int) $a;
    }
}

function switchDefault($a): int
{
    switch ($a) {
        case 1:
        case 2:
            return (int) $a;
        default:
            return (int) $a;
    }
}

function switchNoDefault($a)
{
    switch ($a) {
        case 1:
            return (int) $a;
    }
}

function switchEmptyDefault($a)
{
    switch ($a) {
        case 1:
            return (int) $a;
        default:
    }
}

function switchCaseFallsOut($a)
{
    switch ($a):
        case 1:
            echo 1;
            break;
        default:
            return (int) $a;
    endswitch;
}

function switchColon($a): int
{
    switch ($a):
        case 1:
            return (int) $a;
        default:
            die();
    endswitch;
}

function ifColon($a): int
{
    if ($a):
        return (int) $a;
    elseif ($a > 1):
        return (int) $a;
    else:
        return (int) $a;
    endif;
}

function ifColonNoElse($a)
{
    if ($a):
        return (int) $a;
    endif;
}

function ifColonElseIfFalls($a)
{
    if ($a):
        return (int) $a;
    elseif ($a > 1):
        echo 1;
    else:
        return (int) $a;
    endif;
}

function ifElseIfFalls($a)
{
    if ($a) {
        return (int) $a;
    } elseif ($a > 1) {
        echo 1;
    } else {
        return (int) $a;
    }
}

function ifElseFalls($a)
{
    if ($a) {
        return (int) $a;
    } else {
        echo 1;
    }
}

function ifAll($a): int
{
    if ($a) {
        return (int) $a;
    } elseif ($a > 1) {
        return (int) $a;
    } else {
        throw new \RuntimeException();
    }
}

function whileTrue($a): int
{
    while (true) {
        if ($a) {
            return (int) $a;
        }
    }
}

function whileTrueBreak($a)
{
    while (true) {
        if ($a) {
            return (int) $a;
        }
        break;
    }
}

function whileTrueColon($a): int
{
    while (true):
        if ($a) {
            return (int) $a;
        }
    endwhile;
}

function whileTrueContinueInClosure($a): int
{
    while (true) {
        $f = function () {
            foreach ([1] as $x) {
                continue;
            }
        };
        if ($a) {
            return (int) $a;
        }
    }
}

function whileCondition($a)
{
    while ($a) {
        return (int) $a;
    }
}

function doReturns($a): int
{
    do {
        return (int) $a;
    } while ($a);
}

function doTrue($a): int
{
    do {
        if ($a) {
            return (int) $a;
        }
    } while (true);
}

function doTrueGoto($a)
{
    do {
        if ($a) {
            return (int) $a;
        }
        goto end;
    } while (true);
    end:
}

function doFalls($a)
{
    do {
        if ($a) {
            return (int) $a;
        }
    } while ($a);
}

function neverCall($a): int
{
    if ($a) {
        return (int) $a;
    }
    fail();
}

function yields($a)
{
    if ($a) {
        return (int) $a;
    }
    yield 1;
}

class Methods
{
    public function stop(): never
    {
        exit;
    }

    public static function stopStatic(): never
    {
        exit;
    }

    public function viaThis($a): int
    {
        if ($a) {
            return (int) $a;
        }
        $this->stop();
    }

    public function viaStatic($a): int
    {
        if ($a) {
            return (int) $a;
        }
        self::stopStatic();
    }
}
