<?php

namespace App;

final class Filters
{
    private const FLAG = true;

    public function run(int $i, float $f, bool $b, ?int $n, int|float $u, string $s, array $list)
    {
        $a = filter_var($i, FILTER_VALIDATE_INT);
        $a = \filter_var($i, \FILTER_VALIDATE_INT);
        $a = FILTER_VAR($f, FILTER_VALIDATE_FLOAT);
        $a = filter_var($b, FILTER_VALIDATE_BOOL);
        $a = filter_var(self::FLAG, FILTER_VALIDATE_BOOLEAN);
        $a = filter_var($n, FILTER_VALIDATE_INT);
        $a = filter_var($u, FILTER_VALIDATE_INT);
        $a = filter_var($s, FILTER_VALIDATE_INT);
        $a = filter_var($i, FILTER_VALIDATE_FLOAT);
        $a = filter_var((int) $s, FILTER_VALIDATE_INT);
        $a = filter_var(($i * 2), FILTER_VALIDATE_INT);
        $a = filter_var(value: $i, filter: FILTER_VALIDATE_INT);
        $a = filter_var(strlen($s), FILTER_VALIDATE_INT);
        $a = filter_var(count($list), FILTER_VALIDATE_INT, FILTER_NULL_ON_FAILURE);
        $a = filter_var($i > 1, FILTER_VALIDATE_BOOL);
        $a = filter_var($i, 257);
        $a = filter_var(1.5, FILTER_VALIDATE_FLOAT);
        return $a;
    }
}
-----
<?php

namespace App;

final class Filters
{
    private const FLAG = true;

    public function run(int $i, float $f, bool $b, ?int $n, int|float $u, string $s, array $list)
    {
        $a = $i;
        $a = $i;
        $a = $f;
        $a = $b;
        $a = self::FLAG;
        $a = filter_var($n, FILTER_VALIDATE_INT);
        $a = filter_var($u, FILTER_VALIDATE_INT);
        $a = filter_var($s, FILTER_VALIDATE_INT);
        $a = filter_var($i, FILTER_VALIDATE_FLOAT);
        $a = (int) $s;
        $a = $i * 2;
        $a = $i;
        $a = strlen($s);
        $a = filter_var(count($list), FILTER_VALIDATE_INT, FILTER_NULL_ON_FAILURE);
        $a = $i > 1;
        $a = filter_var($i, 257);
        $a = 1.5;
        return $a;
    }
}
