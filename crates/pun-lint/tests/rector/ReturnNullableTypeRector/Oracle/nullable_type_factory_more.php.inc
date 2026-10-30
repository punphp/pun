<?php

enum Suit
{
    case Hearts;
    case Spades;
}

class Base
{
}

final class Child extends Base
{
}

final class NProbe2
{
    public function thisLast($x)
    {
        if ($x) {
            return new NProbe2();
        }
        return $this;
    }

    public function nonEmptyAndInt(string $s, $x)
    {
        if ($x) {
            return 1;
        }
        return $x ? 'a' . $s : 'b' . $s;
    }

    public function enumCases($x)
    {
        if ($x) {
            return Suit::Hearts;
        }
        return Suit::Spades;
    }

    public function enumCaseAndInt($x)
    {
        if ($x) {
            return Suit::Hearts;
        }
        return 1;
    }

    public function built($x)
    {
        $a = [];
        $a[] = 1;
        if ($x) {
            return 1;
        }
        return $a;
    }

    public function nested()
    {
        return [[1], [2]];
    }

    public function mixedArray()
    {
        return [1, 'a'];
    }

    public function intFloat($x)
    {
        if ($x) {
            return 1;
        }
        return 1.0;
    }

    public function childBase($x)
    {
        if ($x) {
            return new Child();
        }
        return new Base();
    }

    public function anon($x)
    {
        if ($x) {
            return new class extends Base {};
        }
        return 1;
    }

    public function arrayAndString(array $a, $x)
    {
        if ($x) {
            return 'a';
        }
        return $a;
    }

    public function closures($x)
    {
        if ($x) {
            return function () {};
        }
        return 1;
    }

    public function falseOrString(string $s, $x)
    {
        if ($x) {
            return false;
        }
        return $s;
    }

    public function trueFalseString(string $s, $x)
    {
        if ($x) {
            return false;
        }
        if ($s) {
            return true;
        }
        return $s;
    }

    public function sameKeysDifferentValues($x)
    {
        if ($x) {
            return ['a' => 1];
        }
        return ['a' => 2];
    }

    public function stringKeysSameValue($x)
    {
        if ($x) {
            return ['a' => 1];
        }
        return ['b' => 1];
    }
}
