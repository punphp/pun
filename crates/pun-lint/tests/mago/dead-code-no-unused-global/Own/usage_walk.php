<?php

function calls()
{
    global $a, $b, $c, $d, $e;
    static $s = 1, $t = 2;
    foo($a);
    compact($name, '', "b");
    $f = function () {
        return $c;
    };
    $g = fn ($d) => $d;
    $h = new class ($e, named: $t) {
        public function m()
        {
            return $s;
        }
    };
    $i = new class {
    };
}

function classes()
{
    global $a, $b, $c, $d, $e;
    static $s, $t;
    class Inner
    {
        public function m()
        {
            return $a;
        }
    }
    interface InnerInterface
    {
    }
    trait InnerTrait
    {
        public function m()
        {
            return $b;
        }
    }
    enum InnerEnum
    {
        public function m()
        {
            return $c;
        }
    }
    function inner()
    {
        return $d;
    }
}

function indirect()
{
    global $a;
    static $s;
    return ${'a'};
}

function dynamic_global($name)
{
    global $$name, $b;
    static $s;
}

abstract class A
{
    abstract public function m();

    public function n()
    {
        global $a;
        static $s;
        return $a . $s;
    }
}
