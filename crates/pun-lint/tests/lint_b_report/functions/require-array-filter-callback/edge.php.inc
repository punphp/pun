<?php

namespace App;

use SimpleXMLElement;

/**
 * @param array<int|null> $a
 * @param array<bool|null> $b
 * @param array<true|null> $c
 * @param array<0|1> $d
 * @param array<0|'x'> $e
 * @param array<''|'0'|null> $f
 * @param array<non-empty-string|null> $g
 * @param array<non-falsy-string|null> $h
 * @param array<positive-int|0> $i
 * @param array<SimpleXMLElement|null> $j
 * @param array<array{}|array{1}> $k
 * @param array<\stdClass|false> $l
 * @param list<array<int>|null> $m
 * @param iterable<int> $n
 * @param array<1.5|0.0> $o
 * @param array<resource|null> $p
 * @param array<\Closure|null> $q
 * @param array<true> $r
 * @param array<mixed> $s
 * @param ?callable $cb
 * @param array<\UnitEnum|null> $t
 */
function f(array $a, array $b, array $c, array $d, array $e, array $f, array $g, array $h, array $i, array $j, array $k, array $l, array $m, iterable $n, array $o, array $p, array $q, array $r, array $s, ?callable $cb, array $t, int $int, $mixed, array $args)
{
    array_filter($a);
    array_filter($b);
    array_filter($c);
    array_filter($d);
    array_filter($e);
    array_filter($f);
    array_filter($g);
    array_filter($h);
    array_filter($i);
    array_filter($j);
    array_filter($k);
    array_filter($l);
    array_filter($m);
    array_filter($n);
    array_filter($o);
    array_filter($p);
    array_filter($q);
    array_filter($r);
    array_filter($s);
    array_filter($t);
    array_filter($int);
    array_filter($mixed);
    array_filter($a, $cb);
    array_filter($a, mode: ARRAY_FILTER_USE_KEY);
    array_filter(callback: null, array: $a);
    array_filter(array: $a);
    array_filter(...$args);
    array_filter($a, null, 1);
    \array_filter($a);
    ARRAY_FILTER($a);
    array_filter([0, 1]);
    array_filter([null, new \stdClass()]);
    array_filter([]);
    array_filter(['a' => true, 'b' => false]);
    array_filter($a, foo: 1);
}
-----
32: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
33: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
37: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
38: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
40: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
41: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
44: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
45: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
50: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
52: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
53: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
55: Parameter #2 of array_filter() cannot be null to avoid loose comparison semantics (null given).
56: Parameter #2 of array_filter() cannot be null to avoid loose comparison semantics (null given).
57: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
58: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
59: Parameter #2 of array_filter() cannot be null to avoid loose comparison semantics (null given).
60: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
61: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
64: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
