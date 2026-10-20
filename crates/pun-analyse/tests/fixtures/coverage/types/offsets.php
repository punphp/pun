<?php

namespace CoverageOffsets;

class Access implements \ArrayAccess
{
    public function offsetExists($offset): bool
    {
        return true;
    }

    public function offsetGet($offset): mixed
    {
        return 1;
    }

    /**
     * @param string $offset
     * @param int $value
     */
    public function offsetSet($offset, $value): void
    {
    }

    public function offsetUnset($offset): void
    {
    }

    public function self(): void
    {
        $this[1] = 1;
    }
}

final class Closed
{
}

class Open
{
}

interface Shape
{
}

/**
 * @template T
 */
class Box implements \ArrayAccess
{
    public function offsetExists($offset): bool
    {
        return true;
    }

    public function offsetGet($offset): mixed
    {
        return 1;
    }

    /**
     * @param int $offset
     * @param T $value
     */
    public function offsetSet($offset, $value): void
    {
    }

    public function offsetUnset($offset): void
    {
    }
}

/**
 * @param Box<int> $box
 * @param list<int> $list
 * @param non-empty-string $nonEmpty
 * @param string|Access $either
 * @param Shape&\ArrayAccess<int, int> $shape
 */
function offsets(
    string $s,
    string $s2,
    string $s3,
    string $s4,
    string $s5,
    Access $a,
    Closed $closed,
    Open $open,
    Shape $shape,
    \SimpleXMLElement $xml,
    Box $box,
    array $list,
    string $nonEmpty,
    $either,
    int $i,
): void {
    $s['x'] = 'a';
    $s2[] = 'b';
    $s3[0] = [];
    $c = 'abc';
    $c[1] = 'd';
    $c[-1] = 'e';
    $c[2] = 'ef';
    $a[1] = 2;
    $a['k'] = 'v';
    $a[] = 3;
    $closed['x'] = 1;
    $open['x'] = 1;
    $shape['x'] = 1;
    $xml['x'] = 'y';
    $box[1] = 'z';
    $box['k'] = 1;
    $list['k'] = 1;
    $list[] = 1;
    $nonEmpty['k'] = 'x';
    $either['k'] = 'x';
    $i['k'] = 1;
    $s4['k'] .= 'x';
    $s5[] .= 'x';
    $a[1] .= 'x';
    $a['k'] .= 'x';
    $closed['k'] .= 'x';
    $i['k'] .= 'x';
    $s[0] .= 'x';
    $nested = ['a' => 'b'];
    $nested['a']['b'] = 'c';
    $max = [9223372036854775807 => 1];
    $max[] = 2;
    $ref = 'x';
    $a['k'] = &$ref;
}

/**
 * @param int|float $intOrFloat
 * @param Open|string $openOrString
 * @param Open|string $openOrString2
 */
function unions($intOrFloat, $openOrString, $openOrString2, object $o, iterable $it): void
{
    $it['k'] = 1;
    $intOrFloat['k'] = 1;
    $openOrString[0] = 'a';
    $o['k'] = 1;
    $openOrString2[0] .= 'a';
}

/**
 * @template T of int
 * @param T $t
 * @param non-empty-string $nes
 * @param resource $r
 * @param iterable<int> $it
 * @param \Closure(): int $cl
 * @param callable(): int $ca
 */
function powers($t, string $nes, $r, \Closure $cl, callable $ca, iterable $it, \stdClass $o, \stdClass $o2, int $i, array $a, float $f): void
{
    echo ($i ? $i : $f) ** 2;
    echo 2 ** ($i ? $i : $f);
    echo ($i ? $a : 1) ** 2;
    echo $nes ** 2;
    echo $t ** 2;
    echo null ** 2;
    echo $r ** 2;
    echo $cl ** 2;
    echo $ca ** 2;
    echo $it ** 2;
    echo $o ** 2;
    $p = $o ** $o2;
    echo 2 ** ($i ? 1 : $a);
    echo 1 / null;
    echo $a + ($i ? $a : 1);
    $q = ($i ? $a : 1) + ($i ? $a : 's');
}

function binary(string $s, int $i, array $a, mixed $m, \GMP $gmp, float $f, bool $b, ?int $n): void
{
    echo $s . $a;
    echo $s & $s;
    echo $s & $i;
    echo $i | $a;
    echo 'a' ^ 'b';
    echo 'a' ^ 1;
    echo $m & $m;
    echo $s + 1;
    echo $a + $a;
    echo $a + $i;
    echo $i + [1];
    echo $i + 1;
    echo 1 / 0;
    echo $i / 0;
    echo $i / 2;
    echo 5 % 0;
    echo $i % 0;
    echo $i % $a;
    echo $i % 2;
    echo 1 << -1;
    echo $i << $a;
    echo $i >> 2;
    echo $gmp + 1;
    echo $i ** $a;
    echo $f * 2;
    echo $b - 1;
    echo $n + 1;
    echo $a - $a;
    echo 2 ** 3;
    echo 'x' . 1;
    echo $i + $m;
    echo $s + $m;
    echo ($i ? $a : 1) + 1;
    echo ($i ? $a : $s) + $a;
    $i += 'x';
    $i -= 1;
    $i ??= 1;
    $s .= $a;
}
