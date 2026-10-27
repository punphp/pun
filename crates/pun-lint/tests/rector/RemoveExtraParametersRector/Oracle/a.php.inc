<?php

namespace App;

/** @template T */
interface Box
{
    public function get();
}

interface Plain
{
    public function get();
}

/** @template T */
abstract class AbstractBox
{
    abstract public function get();
    public function put($v) {}
}

abstract class PlainAbstract
{
    abstract public function get();
}

function f(\Iterator $i, \Countable $c, \ArrayObject $a, Box $b, Plain $p, AbstractBox $ab, PlainAbstract $pa, ?\ArrayObject $n, \ArrayObject|\SplObjectStorage $u)
{
    $i->current(1);
    $c->count(1);
    $a->count(1);
    $b->get(1);
    $p->get(1);
    $ab->get(1);
    $ab->put(1, 2);
    $pa->get(1);
    $n->count(1);
    $u->count(1);
    $n?->count(1);
    strlen('a', 'b', 'c');
    \strlen('a', 'b');
    array_map(fn() => 1, [], [], []);
    max(1, 2, 3);
    implode(',', [], 3);
    mt_rand(1, 2, 3);
    json_encode(1, 2, 3, 4);
}
-----
<?php

namespace App;

/** @template T */
interface Box
{
    public function get();
}

interface Plain
{
    public function get();
}

/** @template T */
abstract class AbstractBox
{
    abstract public function get();
    public function put($v) {}
}

abstract class PlainAbstract
{
    abstract public function get();
}

function f(\Iterator $i, \Countable $c, \ArrayObject $a, Box $b, Plain $p, AbstractBox $ab, PlainAbstract $pa, ?\ArrayObject $n, \ArrayObject|\SplObjectStorage $u)
{
    $i->current(1);
    $c->count(1);
    $a->count(1);
    $b->get(1);
    $p->get(1);
    $ab->get(1);
    $ab->put(1);
    $pa->get(1);
    $n->count(1);
    $u->count(1);
    $n?->count(1);
    strlen('a');
    \strlen('a');
    array_map(fn() => 1, [], [], []);
    max(1, 2, 3);
    implode(',', []);
    mt_rand(1, 2);
    json_encode(1, 2, 3);
}
