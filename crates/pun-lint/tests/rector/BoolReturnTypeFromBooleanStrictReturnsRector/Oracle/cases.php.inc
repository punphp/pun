<?php

namespace App;

function is_thing($a)
{
    return true;
}

function compare($a, $b)
{
    if ($a) {
        return $a === $b;
    }

    return ($a <=> $b) > 0;
}

function mixed($a)
{
    if ($a) {
        return true;
    }

    return $a instanceof \Countable;
}

function builtin($a)
{
    return is_string($a);
}

function namespaced($a)
{
    return is_thing($a);
}

function builtinNotBool($a)
{
    return strlen($a);
}

function pregMatch($a)
{
    return preg_match('/a/', $a);
}

function inArray($a)
{
    return \in_array($a, [1, 2], true);
}

function logical($a, $b)
{
    return $a and $b xor !$a;
}

function casts($a)
{
    return (bool) $a || (boolean) $a;
}

function spaceship($a, $b)
{
    return $a <=> $b;
}

function maybe($a)
{
    if ($a) {
        return isset($a[1]);
    }
}

final class Checks
{
    public function has($a)
    {
        return !empty($a);
    }
}
-----
<?php

namespace App;

function is_thing($a)
{
    return true;
}

function compare($a, $b): bool
{
    if ($a) {
        return $a === $b;
    }

    return ($a <=> $b) > 0;
}

function mixed($a)
{
    if ($a) {
        return true;
    }

    return $a instanceof \Countable;
}

function builtin($a): bool
{
    return is_string($a);
}

function namespaced($a)
{
    return is_thing($a);
}

function builtinNotBool($a)
{
    return strlen($a);
}

function pregMatch($a)
{
    return preg_match('/a/', $a);
}

function inArray($a): bool
{
    return \in_array($a, [1, 2], true);
}

function logical($a, $b): bool
{
    return $a and $b xor !$a;
}

function casts($a): bool
{
    return (bool) $a || (boolean) $a;
}

function spaceship($a, $b)
{
    return $a <=> $b;
}

function maybe($a)
{
    if ($a) {
        return isset($a[1]);
    }
}

final class Checks
{
    public function has($a): bool
    {
        return !empty($a);
    }
}
