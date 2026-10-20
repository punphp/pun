<?php

namespace StrictOperators;

class Foo
{
    public int $count = 0;

    /**
     * @template T
     * @param T $t
     */
    public function booleans(bool $b, int $i, ?bool $nb, string $s, $m, array $a, $t): void
    {
        if ($b && $i) {
        }
        if ($i || $b) {
        }
        if ($s and $nb) {
        }
        if ($a or $m) {
        }
        if ($t && $b) {
        }
        if (!$i) {
        }
        if (!$b) {
        }
        if (!$m) {
        }
        if (!$nb) {
        }
        if ($this && $b) {
        }
        $x = $i > 0 && $s !== '' || is_int($i);
    }

    /**
     * @template T
     * @param T $t
     * @param numeric-string $ns
     * @param int|string $is
     */
    public function numbers(int $i, float $f, string $s, $t, string $ns, array $a, ?int $ni, int|string $is, \GMP $g, \stdClass $o, mixed $m, bool $b, bool $b2, null $n): void
    {
        $r = $i + $f;
        $r = $s + $i;
        $r = $i - $s;
        $r = $ns * $f;
        $r = $a + $a;
        $r = $a + $i;
        $r = $ni / $i;
        $r = $is % 2;
        $r = $i ** $s;
        $r = $g + 1;
        $r = $o + 1;
        $r = $i - 1;
        $r = $t * 2;
        $r = $b + 1;
        $r = $n + 1;
        $r = -$s;
        $r = +$s;
        $r = -$i;
        $r = -$b;
        $i += $s;
        $f -= $b;
        $s *= 2;
        $i /= $ni;
        $f %= $b;
        $i **= $ns;
        $s++;
        $s--;
        ++$b;
        --$b;
        $ni++;
        $a++;
        $a--;
        $o++;
        $g++;
        $this->count++;
        $f--;
        $ns--;
        $is++;
        $b2--;
    }
}

const SUM = 1 + 2;

const BOTH = true && false;

/**
 * @param non-empty-array<int> $ne
 */
function loose(int|string $is, int|float $n, bool|int $bi, string $s2, array $ne): void
{
    $s2++;
    $r = $ne + $ne;
    $r = $is + 1;
    $r = $n + 1;
    if ($bi || !$bi) {
    }
}

function arrays(array $x, string $s): void
{
    $r = $x + $x;
    $x += $x;
    return;
    $r = $s && $s;
    $r = -$s;
    $r = $s + 1;
}
