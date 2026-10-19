<?php

final class A
{
    public $property;

    public function run($other, $class, $x)
    {
        $this->a(1, 2);
        $other->a(1, 2);
        $other?->b(1, 2);
        $this?->c(1, b: 2);
        $this->$x(1);
        $other::d(1, 2);
        $class::e(1, 2);
        $f = $other->g(...);
        $h = $class::h(...);
        $j = self::j(...);
        $anonymous = new class ($this->k(1, 2)) {
            public function __construct($v)
            {
            }
        };
        $plain = new class {
        };
        $this->m(1, 2);
    }

    private function a($used, $unused)
    {
        return $used;
    }

    private function b($used, $unused)
    {
        return $used;
    }

    private function c($used, $unused)
    {
        return $used;
    }

    private function d($used, $unused)
    {
        return $used;
    }

    private function e($used, $unused)
    {
        return $used;
    }

    private function f($used, $unused)
    {
        return $used;
    }

    private function g($used, $unused)
    {
        return $used;
    }

    private function h($used, $unused)
    {
        return $used;
    }

    private function i($used, $unused)
    {
        return $used;
    }

    private function j($used, $unused)
    {
        return $used;
    }

    private function k($used, $unused)
    {
        return $used;
    }

    private function m($used, $unused)
    {
        return $used;
    }
}

class Inner
{
    public function run()
    {
        $this->m(1, 2);
    }

    private function m($used, $unused)
    {
        return $used;
    }
}
-----
<?php

final class A
{
    public $property;

    public function run($other, $class, $x)
    {
        $this->a(1);
        $other->a(1, 2);
        $other?->b(1, 2);
        $this?->c(1, b: 2);
        $this->$x(1);
        $other::d(1, 2);
        $class::e(1, 2);
        $f = $other->g(...);
        $h = $class::h(...);
        $j = self::j(...);
        $anonymous = new class ($this->k(1)) {
            public function __construct($v)
            {
            }
        };
        $plain = new class {
        };
        $this->m(1);
    }

    private function a($used)
    {
        return $used;
    }

    private function b($used)
    {
        return $used;
    }

    private function c($used)
    {
        return $used;
    }

    private function d($used)
    {
        return $used;
    }

    private function e($used)
    {
        return $used;
    }

    private function f($used)
    {
        return $used;
    }

    private function g($used)
    {
        return $used;
    }

    private function h($used)
    {
        return $used;
    }

    private function i($used)
    {
        return $used;
    }

    private function j($used, $unused)
    {
        return $used;
    }

    private function k($used)
    {
        return $used;
    }

    private function m($used)
    {
        return $used;
    }
}

class Inner
{
    public function run()
    {
        $this->m(1);
    }

    private function m($used)
    {
        return $used;
    }
}
