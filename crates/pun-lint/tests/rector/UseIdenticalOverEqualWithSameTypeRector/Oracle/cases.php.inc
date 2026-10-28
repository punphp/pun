<?php

namespace App;

enum E { case A; }

class C
{
    private int $p = 0;
    private array $arr = [];

    public function run(int $i, string $s, ?string $ns, array $a, array $b, bool $flag, float $f, $untyped, E $e, int|string $u, iterable $it, callable $cb, null $nul)
    {
        $r1 = ($i + 1) == 2;
        $r2 = $s == 'x';
        $r3 = $ns == 'x';
        $r4 = $a == $b;
        $r5 = $a == [];
        $r6 = [] == [];
        $r7 = $flag == false;
        $r8 = $f == 1.5;
        $r9 = $i == 1.0;
        $r10 = $untyped == 1;
        $r11 = $e == E::A;
        $r12 = $u == $u;
        $r13 = count($a) == 0;
        $r14 = strlen($s) != 3;
        $r15 = $this->p == 1;
        $r16 = $this->arr == [];
        $r17 = $s <> 'y';
        $r18 = $s /* c */ == /* d */ 'x';
        $r19 = (string) $i == $s;
        $r20 = $i == null;
        $r21 = null == null;
        $r22 = $nul == null;
        $r23 = 'a' . $s == "b{$s}";
        $r24 = $flag == ($i > 1);
        $r25 = $i == ($flag ? 1 : 2);
        $r26 = ($s) == ('x');
        $r27 = $a['k'] == 1;
        $r28 = trim($s) == '';
        $r29 = substr($s, 1) == 'x';
        $r30 = $it == $it;
        $r31 = __LINE__ == 3;
        $r32 = PHP_VERSION_ID == 80000;
        $r33 = \strtolower($s) == 'x';
        $r34 = $i
            == 2;
        $r35 = json_encode($a) == '[]';
        $r36 = $s == self::class;
        $r37 = $this->m() == 'x';
        $r38 = intdiv($i, 2) == 1;
        $r39 = $i % 2 == 0;
        $r40 = $i / 2 == 1;
        $r41 = $f * 2 == 1;
        $r42 = [1, 2] == [1, 2];
        $r43 = [$i] == [$i];
        $r44 = ($i <=> 1) == 0;
        $r45 = (int) $s == $i;
        $r46 = random_int(1, 2) == 1;
        $r47 = $s[0] == 'x';
        $r48 = ord($s) == 1;
    }

    public function m(): string { return ''; }
}
-----
<?php

namespace App;

enum E { case A; }

class C
{
    private int $p = 0;
    private array $arr = [];

    public function run(int $i, string $s, ?string $ns, array $a, array $b, bool $flag, float $f, $untyped, E $e, int|string $u, iterable $it, callable $cb, null $nul)
    {
        $r1 = $i + 1 === 2;
        $r2 = $s === 'x';
        $r3 = $ns == 'x';
        $r4 = $a == $b;
        $r5 = $a == [];
        $r6 = [] === [];
        $r7 = $flag === false;
        $r8 = $f === 1.5;
        $r9 = $i == 1.0;
        $r10 = $untyped == 1;
        $r11 = $e == E::A;
        $r12 = $u == $u;
        $r13 = count($a) === 0;
        $r14 = strlen($s) !== 3;
        $r15 = $this->p === 1;
        $r16 = $this->arr == [];
        $r17 = $s !== 'y';
        $r18 = $s === 'x';
        $r19 = (string) $i === $s;
        $r20 = $i == null;
        $r21 = null === null;
        $r22 = $nul === null;
        $r23 = 'a' . $s === "b{$s}";
        $r24 = $flag === $i > 1;
        $r25 = $i === ($flag ? 1 : 2);
        $r26 = $s === 'x';
        $r27 = $a['k'] == 1;
        $r28 = trim($s) === '';
        $r29 = substr($s, 1) === 'x';
        $r30 = $it == $it;
        $r31 = __LINE__ === 3;
        $r32 = PHP_VERSION_ID === 80000;
        $r33 = \strtolower($s) === 'x';
        $r34 = $i === 2;
        $r35 = json_encode($a) == '[]';
        $r36 = $s == self::class;
        $r37 = $this->m() === 'x';
        $r38 = intdiv($i, 2) === 1;
        $r39 = $i % 2 === 0;
        $r40 = $i / 2 == 1;
        $r41 = $f * 2 == 1;
        $r42 = [1, 2] === [1, 2];
        $r43 = [$i] === [$i];
        $r44 = ($i <=> 1) === 0;
        $r45 = (int) $s === $i;
        $r46 = random_int(1, 2) === 1;
        $r47 = $s[0] == 'x';
        $r48 = ord($s) === 1;
    }

    public function m(): string { return ''; }
}
