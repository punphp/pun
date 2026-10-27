<?php

namespace App;

final class Maths
{
    public function run(int $i, float $f, int|float $u, string $s, ?int $n, array $list)
    {
        $a = $i + 0;
        $a = $i - 0;
        $a = $f - 0;
        $a = $f + 0;
        $a = 0 + $i;
        $a = 0 - $i;
        $a = 0 - $f;
        $a = 0 - abs($i);
        $a = $f * 1;
        $a = 1 * $f;
        $a = $f / 1;
        $a = $i / 1;
        $a = 1 / $i;
        $a = $u * 1;
        $a = $n * 1;
        $a = $s * 1;
        $a = strlen($s) * 1;
        $a = count($list) + 0;
        $a = 1 * ($i + $f);
        $a = 1 * ($i);
        $a = 1*(  $i - 2  );
        $a = ($i * 1);
        $a = 1 * 1;
        $a = 0 + 0;
        $a = 0x1 * $i;
        $a = $i * 1.0;
        $a = $i + 0.0;
        $i += 0;
        $i -= 0;
        $f -= 0;
        $f += 0;
        $f *= 1;
        $f /= 1;
        $u *= 1;
        $x = '5';
        $x += 0;
        $y = 5;
        $y *= 1;
        foo($i *= 1);
        $a = $i <=> 0 + $i;
        $a = $i . 0 + $i;
        return $a;
    }
}
-----
<?php

namespace App;

final class Maths
{
    public function run(int $i, float $f, int|float $u, string $s, ?int $n, array $list)
    {
        $a = $i;
        $a = $i;
        $a = $f;
        $a = $f + 0;
        $a = $i;
        $a = -$i;
        $a = 0 - $f;
        $a = -abs($i);
        $a = $f;
        $a = $f;
        $a = $f;
        $a = $i;
        $a = 1 / $i;
        $a = $u * 1;
        $a = $n * 1;
        $a = $s * 1;
        $a = strlen($s);
        $a = count($list);
        $a = ($i + $f);
        $a = $i;
        $a = ($i - 2);
        $a = ($i);
        $a = 1;
        $a = 0;
        $a = $i;
        $a = $i * 1.0;
        $a = $i + 0.0;
        $i;
        $i;
        $f;
        $f += 0;
        $f;
        $f;
        $u *= 1;
        $x = '5';
        $x += 0;
        $y = 5;
        $y;
        foo($i);
        $a = $i <=> $i;
        $a = $i . $i;
        return $a;
    }
}
