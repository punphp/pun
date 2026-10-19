<?php

class Base
{
    private const A = 1;
    public const B = 2;
    final public const C = 3;
    private static $p;
    public static $q;
    private $instance;

    private static function m() {}
    public static function n() {}
    final public static function o() {}
    private function inst() {}

    public function run()
    {
        static::A;
        static::B;
        static::C;
        static::$p;
        static::$q;
        static::$instance;
        static::m();
        static::N();
        static::o();
        static::inst();
        static::class;
        static::missing();
        $f = function () {
            return static::m();
        };
    }
}

final class Fin
{
    public const B = 2;
    public static function n() {}
    public function run()
    {
        return [static::B, static::n(), STATIC::B];
    }
}
-----
<?php

class Base
{
    private const A = 1;
    public const B = 2;
    final public const C = 3;
    private static $p;
    public static $q;
    private $instance;

    private static function m() {}
    public static function n() {}
    final public static function o() {}
    private function inst() {}

    public function run()
    {
        self::A;
        static::B;
        self::C;
        self::$p;
        static::$q;
        static::$instance;
        self::m();
        static::N();
        self::o();
        static::inst();
        static::class;
        static::missing();
        $f = function () {
            return self::m();
        };
    }
}

final class Fin
{
    public const B = 2;
    public static function n() {}
    public function run()
    {
        return [self::B, self::n(), self::B];
    }
}
