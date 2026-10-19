<?php

function f0(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            return $v;
        }
    }
    return;
}

function f1(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        foo();
        if ($k === $key) {
            return $v;
        }
    }
    return null;
}

function f2(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        echo 1;
    }
    return null;
}

function f3(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k) {
            return $v;
        }
    }
    return null;
}

function f4(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k == $key) {
            return $v;
        }
    }
    return null;
}

function f5(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            $r += $v;
        }
    }
    return $r;
}

function f6(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            $r = &$v;
        }
    }
    return $r;
}

function f7(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            echo $v;
        }
    }
    return null;
}

function f8(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            foo();
        }
    }
    return null;
}

function f9(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            $r = $v;
        }
    }
    return $r;
}

function f10(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            return $v;
        }
    }
    foo();
}

function f11(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($key === $k) {
            $r = $v;
        }
    }
    return $r;
}

class A
{
    public $value {
        get {
            foreach ([1, 2] as $k => $v) {
                if ($k === 1) {
                    return $v;
                }
            }
            return null;
        }
    }
}

function g0(array $items, $key)
{
    foreach ($items as $v) {
        if ($v === $key) {
            return $v;
        }
    }
    return null;
}

function g1(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            $r = $key;
        }
    }
    return $r;
}

function g2(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            return $key;
        }
    }
    return $r;
}

function g3(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($v === $key) {
            return $v;
        }
    }
    return $r;
}
-----
<?php

function f0(array $items, $key, $r)
{
    return $items[$key] ?? $key;
}

function f1(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        foo();
        if ($k === $key) {
            return $v;
        }
    }
    return null;
}

function f2(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        echo 1;
    }
    return null;
}

function f3(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k) {
            return $v;
        }
    }
    return null;
}

function f4(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k == $key) {
            return $v;
        }
    }
    return null;
}

function f5(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            $r += $v;
        }
    }
    return $r;
}

function f6(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            $r = &$v;
        }
    }
    return $r;
}

function f7(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            echo $v;
        }
    }
    return null;
}

function f8(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            foo();
        }
    }
    return null;
}

function f9(array $items, $key, $r)
{
    $r = $items[$key] ?? $r;
    return $r;
}

function f10(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            return $v;
        }
    }
    foo();
}

function f11(array $items, $key, $r)
{
    $r = $items[$key] ?? $r;
    return $r;
}

class A
{
    public $value {
        get {
            foreach ([1, 2] as $k => $v) {
                if ($k === 1) {
                    return $v;
                }
            }
            return null;
        }
    }
}

function g0(array $items, $key)
{
    foreach ($items as $v) {
        if ($v === $key) {
            return $v;
        }
    }
    return null;
}

function g1(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            $r = $key;
        }
    }
    return $r;
}

function g2(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($k === $key) {
            return $key;
        }
    }
    return $r;
}

function g3(array $items, $key, $r)
{
    foreach ($items as $k => $v) {
        if ($v === $key) {
            return $v;
        }
    }
    return $r;
}
