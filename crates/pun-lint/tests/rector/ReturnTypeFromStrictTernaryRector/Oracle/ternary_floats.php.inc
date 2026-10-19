<?php

final class TernaryFloats
{
    public function f($x)
    {
        return $x ? 1.5 : 2.5;
    }

    public function v(): void
    {
    }

    public function g($x)
    {
        if ($x) {
            return $this->v();
        }
        return $x ? 1 : 2;
    }
}
-----
<?php

final class TernaryFloats
{
    public function f($x): float
    {
        return $x ? 1.5 : 2.5;
    }

    public function v(): void
    {
    }

    public function g($x)
    {
        if ($x) {
            return $this->v();
        }
        return $x ? 1 : 2;
    }
}
