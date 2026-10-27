<?php

namespace App;

enum Suit
{
    case Hearts;
    case Spades;
}

function v(): void
{
}

/** @return never */
function n()
{
    throw new \Exception();
}

/**
 * @param true $t
 * @param false $f
 * @param array-key $k
 * @param numeric $num
 * @param scalar $s
 * @param resource $r
 * @param \ArrayObject<int, string> $g
 * @param 1.5 $lit
 * @param \Closure(): void $cl
 */
function f(string $m, bool $b, float $fl, iterable $i, callable $c, object $o, Suit $suit, $t, $f, $k, $num, $s, $r, $g, $lit, $cl, int|float $u, ?Suit $ns)
{
    $b->$m();
    $fl->$m();
    $i->$m();
    $c->$m();
    $o->$m();
    $suit->$m();
    $t->$m();
    $f->$m();
    $k->$m();
    $num->$m();
    $s->$m();
    $r->$m();
    $g->$m();
    $lit->$m();
    $cl->$m();
    $u->$m();
    $ns?->$m();
    Suit::Hearts->$m();
    $e = Suit::Spades;
    $e->$m();
    v()->$m();
    $x = [1, 2];
    $x->$m();
}

function g(string $m)
{
    n()->$m();
}

function h(string $m)
{
    return;
    $m->$m();
}
-----
34: Variable method call on bool.
35: Variable method call on float.
36: Variable method call on iterable.
37: Variable method call on callable.
38: Variable method call on object.
39: Variable method call on App\Suit.
40: Variable method call on true.
41: Variable method call on false.
42: Variable method call on (int|string).
43: Variable method call on float|int|string.
44: Variable method call on bool|float|int|string.
45: Variable method call on resource.
46: Variable method call on ArrayObject<int, string>.
47: Variable method call on float.
48: Variable method call on Closure.
49: Variable method call on float|int.
50: Variable method call on App\Suit.
51: Variable method call on App\Suit::Hearts.
53: Variable method call on App\Suit::Spades.
54: Variable method call on null.
56: Variable method call on array<int, int>.
61: Variable method call on never.
