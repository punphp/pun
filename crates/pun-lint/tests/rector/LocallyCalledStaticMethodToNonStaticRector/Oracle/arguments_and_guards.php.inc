<?php

interface HasRun
{
    public function go();
}

abstract class Base
{
    public static function base()
    {
    }
}

final class B extends Base implements HasRun
{
    public function go()
    {
        self::multi(
            1,
            // comment
            2,
        );
        self::lines(
            'a',
            'b'
        );
        $f = self::callable(...);
        self::base();
        static::later();
        self::Mixed(1);
        $o = new class {
            public function x()
            {
                return self::inner();
            }

            private static function inner()
            {
                return 1;
            }
        };
    }

    private static function multi($a, $b)
    {
    }

    private static function lines($a, $b)
    {
    }

    private static function callable()
    {
    }

    private static function base()
    {
    }

    private static function later()
    {
    }

    private static function mixed($x)
    {
    }
}
-----
<?php

interface HasRun
{
    public function go();
}

abstract class Base
{
    public static function base()
    {
    }
}

final class B extends Base implements HasRun
{
    public function go()
    {
        $this->multi(
            1,
            // comment
            2
        );
        $this->lines('a', 'b');
        $f = $this->callable(...);
        self::base();
        $this->later();
        $this->mixed(1);
        $o = new class {
            public function x()
            {
                return $this->inner();
            }

            private function inner()
            {
                return 1;
            }
        };
    }

    private function multi($a, $b)
    {
    }

    private function lines($a, $b)
    {
    }

    private function callable()
    {
    }

    private static function base()
    {
    }

    private function later()
    {
    }

    private function mixed($x)
    {
    }
}
