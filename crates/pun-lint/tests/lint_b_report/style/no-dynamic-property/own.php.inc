<?php

namespace N;

class Foo {}
class Std extends \stdClass {}
class Xml extends \SimpleXMLElement {}

function f(Foo $foo, ?Foo $maybe, \stdClass $std, Std $sub, Xml $xml, $mixed, Foo|\stdClass $either, string $s, $p) {
    $str = $s;
    $lit = 'a';
    $either2 = rand() ? 'a' : 'b';
    $foo->$str;
    $foo->$lit;
    $foo->$either2;
    $foo->{'x'};
    $foo->{$str . 'x'};
    $foo->$s;
    $foo->$p;
    $maybe?->$str;
    $std->$str;
    $sub->$str;
    $xml->$str;
    $mixed->$str;
    $either->$str;
    $foo->$str = 1;
    Foo::$$str;
    Foo::${'x'};
    Foo::${$lit};
    $foo::$$str;
    \stdClass::$$str;
}
-----
13: Variable property access on N\Foo.
17: Variable property access on N\Foo.
18: Variable property access on N\Foo.
19: Variable property access on N\Foo.
20: Variable property access on N\Foo.
24: Variable property access on mixed.
26: Variable property access on N\Foo.
