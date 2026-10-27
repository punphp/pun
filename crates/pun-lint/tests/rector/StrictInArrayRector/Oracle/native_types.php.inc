<?php

namespace N;

class C
{
    public function run(int $i, string $s, ?string $n, int|string $u, array $a, $m, bool $b)
    {
        $x = in_array($i, [1, 2]);
        $x = in_array($s, ['a' => 'x', 'b' => 'y']);
        $x = in_array($n, ['a', 'b']);
        $x = in_array($u, [1, 'a']);
        $x = in_array($i, []);
        $x = in_array($s, $a);
        $x = in_array($m, ['a']);
        $x = in_array($i, ['1']);
        $x = in_array($b, [true]);
        $x = in_array($i, [1, 2], false);
        $x = \in_array($i, [1]);
        $x = IN_ARRAY($i, [1]);
        $x = in_array($i, [1, 2],);
        $x = in_array(...[$i, [1]]);
    }
}
-----
<?php

namespace N;

class C
{
    public function run(int $i, string $s, ?string $n, int|string $u, array $a, $m, bool $b)
    {
        $x = in_array($i, [1, 2], true);
        $x = in_array($s, ['a' => 'x', 'b' => 'y'], true);
        $x = in_array($n, ['a', 'b'], true);
        $x = in_array($u, [1, 'a'], true);
        $x = in_array($i, [], true);
        $x = in_array($s, $a);
        $x = in_array($m, ['a']);
        $x = in_array($i, ['1']);
        $x = in_array($b, [true], true);
        $x = in_array($i, [1, 2], false);
        $x = \in_array($i, [1], true);
        $x = IN_ARRAY($i, [1], true);
        $x = in_array($i, [1, 2], true,);
        $x = in_array(...[$i, [1]]);
    }
}
