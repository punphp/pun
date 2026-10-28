<?php

namespace Oracle\ReturnBool2;

final class Item
{
}

final class Types
{
    public function nullableObject(?Item $item)
    {
        if ($item) {
            return true;
        }
        return false;
    }

    public function nullableInt(?int $value)
    {
        if ($value) {
            return true;
        }
        return false;
    }

    public function boolValue(bool $value)
    {
        if ($value) {
            return true;
        }
        return false;
    }

    public function coalesce(?Item $a, ?Item $b)
    {
        if ($a ?? $b) {
            return true;
        }
        return false;
    }

    public function constantNullable($flag)
    {
        $value = $flag ? 'yes' : null;
        if ($value) {
            return true;
        }
        return false;
    }

    public function arrayNullable($flag)
    {
        $value = $flag ? [1] : null;
        if ($value) {
            return true;
        }
        return false;
    }

    public function negatedNullable(?Item $item)
    {
        if (! $item) {
            return false;
        }
        return true;
    }

    public function braceless($a)
    {
        if ($a) return true;
        return false;
    }

    public function upperCase($a)
    {
        if ($a) {
            return TRUE;
        }
        return \false;
    }

    public function twoPairs($a, $b)
    {
        if ($a) {
            return true;
        }
        if ($b) {
            return true;
        }
        return false;
    }

    public function trailingComment($a)
    {
        if ($a) {
            return true;
            // after
        }
        return false;
    }

    public function afterStatement($a)
    {
        $b = $a;
        // about the if
        if ($b) {
            /* inner */
            return true;
        }
        // about the return
        return false;
    }

    public function classString()
    {
        $value = rand(0, 1) ? Item::class : null;
        if ($value) {
            return true;
        }
        return false;
    }
}

function oracle_return_bool_null()
{
    $value = null;
    if ($value) {
        return true;
    }
    return false;
}

function oracle_return_bool_unknown(\Oracle\Missing $missing)
{
    if ($missing->value()) {
        return true;
    }
    return false;
}
-----
<?php

namespace Oracle\ReturnBool2;

final class Item
{
}

final class Types
{
    public function nullableObject(?Item $item)
    {
        return $item !== null;
    }

    public function nullableInt(?int $value)
    {
        return (bool) $value;
    }

    public function boolValue(bool $value)
    {
        return $value;
    }

    public function coalesce(?Item $a, ?Item $b)
    {
        return ($a ?? $b) !== null;
    }

    public function constantNullable($flag)
    {
        $value = $flag ? 'yes' : null;
        return $value !== null;
    }

    public function arrayNullable($flag)
    {
        $value = $flag ? [1] : null;
        return $value !== null;
    }

    public function negatedNullable(?Item $item)
    {
        return $item !== null;
    }

    public function braceless($a)
    {
        return (bool) $a;
    }

    public function upperCase($a)
    {
        return (bool) $a;
    }

    public function twoPairs($a, $b)
    {
        if ($a) {
            return true;
        }
        return (bool) $b;
    }

    public function trailingComment($a)
    {
        if ($a) {
            return true;
            // after
        }
        return false;
    }

    public function afterStatement($a)
    {
        $b = $a;
        // about the if
        // about the return
        /* inner */
        return (bool) $b;
    }

    public function classString()
    {
        $value = rand(0, 1) ? Item::class : null;
        return (bool) $value;
    }
}

function oracle_return_bool_null()
{
    $value = null;
    return (bool) $value;
}

function oracle_return_bool_unknown(\Oracle\Missing $missing)
{
    return (bool) $missing->value();
}
