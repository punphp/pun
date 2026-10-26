<?php

final class ArithmeticOnMixed
{
    public function run($o, mixed $m, int $i)
    {
        $a = fn ($s) => $s + 1;
        $b = fn ($s) => $s - 1;
        $c = fn ($s) => $s * 2;
        $d = fn ($s) => $s / 2;
        $e = fn ($s) => $s % 2;
        $f = fn ($s) => $s ** 2;
        $g = fn ($carry, $record) => $carry + $record->delete();
        $h = fn ($x, $y) => $x + $y;
        $j = fn () => $m + 1;
        $k = fn ($s) => -$s;
        $l = fn ($s) => $s + 1.5;
        $n = fn ($s) => 1 + $s;
        $p = fn ($s) => $s << 1;
        $q = fn ($s) => $s & 1;
        $r = fn () => $o->x + 1;
        $t = fn () => $i + $o->count;
        $u = fn ($s) => ($s + 1) * 2;
        $v = fn (int $s) => $s + 1;
        $w = fn ($s) => 2 ** $s;
        $x = fn ($s) => $s >> 1.5;
        $y = fn ($s) => $s & 1.5;
        $z = fn ($s) => ($s + 1) | 1;
        $aa = fn ($s) => ($s + 1) ^ $s;
    }
}
-----
<?php

final class ArithmeticOnMixed
{
    public function run($o, mixed $m, int $i)
    {
        $a = fn ($s): int|float => $s + 1;
        $b = fn ($s): int|float => $s - 1;
        $c = fn ($s): int|float => $s * 2;
        $d = fn ($s): int|float => $s / 2;
        $e = fn ($s): int => $s % 2;
        $f = fn ($s): float|int => $s ** 2;
        $g = fn ($carry, $record): float|int|array => $carry + $record->delete();
        $h = fn ($x, $y): float|int|array => $x + $y;
        $j = fn (): int|float => $m + 1;
        $k = fn ($s): int|float => -$s;
        $l = fn ($s): float => $s + 1.5;
        $n = fn ($s): int|float => 1 + $s;
        $p = fn ($s): int|float => $s << 1;
        $q = fn ($s): int => $s & 1;
        $r = fn (): int|float => $o->x + 1;
        $t = fn (): int|float => $i + $o->count;
        $u = fn ($s): int|float => ($s + 1) * 2;
        $v = fn (int $s): int => $s + 1;
        $w = fn ($s) => 2 ** $s;
        $x = fn ($s): int => $s >> 1.5;
        $y = fn ($s): int => $s & 1.5;
        $z = fn ($s): int => ($s + 1) | 1;
        $aa = fn ($s): int => ($s + 1) ^ $s;
    }
}
