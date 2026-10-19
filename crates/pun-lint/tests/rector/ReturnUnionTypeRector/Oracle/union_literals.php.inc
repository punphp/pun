<?php

final class UnionLiterals
{
    public function floats($x, $y)
    {
        if ($x) {
            return 1.5;
        }
        if ($y) {
            return 2.5;
        }
        return 'a';
    }

    public function general($x)
    {
        if ($x) {
            return 1;
        }
        return (int) $x;
    }

    public function mixed($x)
    {
        if ($x) {
            return $x;
        }
        return 1;
    }
}
-----
<?php

final class UnionLiterals
{
    public function floats($x, $y): float|string
    {
        if ($x) {
            return 1.5;
        }
        if ($y) {
            return 2.5;
        }
        return 'a';
    }

    public function general($x)
    {
        if ($x) {
            return 1;
        }
        return (int) $x;
    }

    public function mixed($x)
    {
        if ($x) {
            return $x;
        }
        return 1;
    }
}
