<?php

class A
{
    public function run()
    {
        self::a();
        static::b(1, 2);
        A::c();
        $f = fn() => self::d();
        $g = function () { return static::e(); };
        $h = static fn() => self::f();
    }

    #[Foo]
    private static function a()
    {
    }

    static private function b($x, $y)
    {
    }

    /** doc */
    final private static function c()
    {
    }

    private static function d()
    {
    }

    private static function e()
    {
    }

    private static function f()
    {
        return 1;
    }
}
-----
<?php

class A
{
    public function run()
    {
        $this->a();
        $this->b(1, 2);
        A::c();
        $f = fn() => $this->d();
        $g = function () { return $this->e(); };
        $h = static fn() => self::f();
    }

    #[Foo]
    private function a()
    {
    }

    private function b($x, $y)
    {
    }

    /** doc */
    final private static function c()
    {
    }

    private function d()
    {
    }

    private function e()
    {
    }

    private static function f()
    {
        return 1;
    }
}
