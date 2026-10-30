<?php

namespace App\Noop;

/**
 * @template T of Foo = Bar
 * @phpstan-type X Foo
 * @phpstan-import-type Y from Foo
 * @method Foo run(Bar $a)
 */
final class Docs
{
    /**
     * @\Foo\Bar
     */
    public $a;

    /**
     * @param Foo[Bar] $a
     * @param object{a: Foo} $b
     * @param Foo<Bar> $g
     * @param callable (int): void $x
     * @return ($a is Foo ? Bar : null)
     * @phpstan-assert Foo $b
     */
    public function run($a, $b, $g, $x)
    {
    }
}
