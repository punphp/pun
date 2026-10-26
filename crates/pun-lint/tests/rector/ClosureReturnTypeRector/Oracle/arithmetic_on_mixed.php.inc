<?php

final class MixedMath
{
    public function plus($s)
    {
        return $s + 1;
    }

    public function both($a, $b)
    {
        return $a + $b;
    }

    public function closure()
    {
        return function ($s) {
            return $s * 2;
        };
    }

    public function pow($s)
    {
        return $s ** 2;
    }
}
-----
<?php

final class MixedMath
{
    public function plus($s)
    {
        return $s + 1;
    }

    public function both($a, $b)
    {
        return $a + $b;
    }

    public function closure()
    {
        return function ($s): int|float {
            return $s * 2;
        };
    }

    public function pow($s)
    {
        return $s ** 2;
    }
}
