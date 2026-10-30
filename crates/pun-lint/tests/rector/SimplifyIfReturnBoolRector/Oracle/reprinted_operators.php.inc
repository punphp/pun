<?php

namespace Oracle\ReturnBool;

final class Operators
{
    public function arithmetic($a, $b)
    {
        if (($a + 1) * 2 / 3 % 4 - $b ** 2 . 'x' > 1) {
            return false;
        }

        return true;
    }

    public function bits($a, $b)
    {
        if ($a & 1 | $b ^ 3 << 1 >> 2) {
            return false;
        }
        return true;
    }

    public function logic($a, $b, $c)
    {
        if ($a && $b || $c and $a or $b xor $c) {
            return false;
        }
        return true;
    }

    public function compare($a, $b)
    {
        if ($a == $b && $a === $b && $a < $b && $a <= $b && $a >= $b && ($a <=> $b) && ($a ?? $b)) {
            return false;
        }
        return true;
    }

    public function multiline($a, $b)
    {
        if ($a
            && $b) {
            return true;
        }
        return false;
    }

    public function instance($a)
    {
        if ($a instanceof \stdClass) {
            return false;
        }
        return true;
    }

    public function angled($a, $b)
    {
        if ($a <> $b) {
            return false;
        }
        return true;
    }

    public function identicalLeft($a, $b)
    {
        if (($a || $b) === ($b && $a)) {
            return false;
        }
        return true;
    }

    public function stringBang($a)
    {
        if ($a === '!=') {
            return false;
        }
        return true;
    }

    public function notEqualReturnFalse($a, $b)
    {
        if ($a != $b) {
            return false;
        }
        return $b;
    }

    public function compareInsideAnd($a, $b)
    {
        if ($a !== $b && $b) {
            return false;
        }
        return true;
    }
}
-----
<?php

namespace Oracle\ReturnBool;

final class Operators
{
    public function arithmetic($a, $b)
    {
        return !(($a + 1) * 2 / 3 % 4 - $b ** 2 . 'x' > 1);
    }

    public function bits($a, $b)
    {
        return !($a & 1 | $b ^ 3 << 1 >> 2);
    }

    public function logic($a, $b, $c)
    {
        return !($a && $b || $c and $a or $b xor $c);
    }

    public function compare($a, $b)
    {
        return !($a == $b && $a === $b && $a < $b && $a <= $b && $a >= $b && $a <=> $b && ($a ?? $b));
    }

    public function multiline($a, $b)
    {
        return $a
            && $b;
    }

    public function instance($a)
    {
        return !$a instanceof \stdClass;
    }

    public function angled($a, $b)
    {
        return $a == $b;
    }

    public function identicalLeft($a, $b)
    {
        return ($a || $b) !== ($b && $a);
    }

    public function stringBang($a)
    {
        if ($a === '!=') {
            return false;
        }
        return true;
    }

    public function notEqualReturnFalse($a, $b)
    {
        if ($a != $b) {
            return false;
        }
        return $b;
    }

    public function compareInsideAnd($a, $b)
    {
        if ($a !== $b && $b) {
            return false;
        }
        return true;
    }
}
