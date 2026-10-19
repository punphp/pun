<?php

namespace App;

class Halter
{
    public static function halt(): never
    {
        exit;
    }
}

final class UnionMoreFree
{
    public function named($x)
    {
        if ($x) {
            return 1;
        }
        Halter::halt();
    }

    public function anonymous($x)
    {
        $o = new class($x) {
            public function __construct($x)
            {
            }
        };
        if ($x) {
            return 1;
        }
        return 'a';
    }

    public function floats($x)
    {
        if ($x) {
            return 1.5;
        }
        return 'a';
    }
}
-----
<?php

namespace App;

class Halter
{
    public static function halt(): never
    {
        exit;
    }
}

final class UnionMoreFree
{
    public function named($x)
    {
        if ($x) {
            return 1;
        }
        Halter::halt();
    }

    public function anonymous($x): int|string
    {
        $o = new class($x) {
            public function __construct($x)
            {
            }
        };
        if ($x) {
            return 1;
        }
        return 'a';
    }

    public function floats($x): float|string
    {
        if ($x) {
            return 1.5;
        }
        return 'a';
    }
}
