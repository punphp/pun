<?php

class C implements ArrayAccess, Countable
{
    public function offsetExists($o): bool
    {
        return true;
    }

    public function offsetGet($o): mixed
    {
        return 1;
    }

    public function offsetSet($o, $v): void
    {
    }

    public function offsetUnset($o): void
    {
    }

    public function count(): int
    {
        return 1;
    }

    public function __clone()
    {
    }

    public function run(?C $b, int|C $c, C&Countable $d, (C&Countable)|null $e, object $f, mixed $g, iterable $h, $x)
    {
        $b[0];
        $c[0];
        $d[0];
        $e[0];
        $f[0];
        $g[0];
        $h[0];
        clone $b;
        if ($x)
            foo() + bar();
    }
}
