<?php

final class UnionSilent
{
    public function braceless($x)
    {
        if ($x) return 1; else return 'a';
    }

    public function call($x)
    {
        foo();
        if ($x) {
            return 1;
        }
        return 'a';
    }
}
-----
<?php

final class UnionSilent
{
    public function braceless($x): int|string
    {
        if ($x) return 1; else return 'a';
    }

    public function call($x): int|string
    {
        foo();
        if ($x) {
            return 1;
        }
        return 'a';
    }
}
