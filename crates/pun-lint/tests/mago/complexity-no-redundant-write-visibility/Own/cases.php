<?php

class A
{
    public public(set) string $a;
    protected protected(set) string $b;
    private private(set) string $c;
    public private(set) string $d;
    public(set) string $e;
    final public public(set) readonly string $f;
    public public(set) string $g { get => 'x'; }

    public function __construct(public public(set) string $h) {}
}
