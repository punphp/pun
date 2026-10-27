<?php

namespace Tests;

use PHPUnit\Framework\TestCase;

class SomeTest extends TestCase
{
    public function testA()
    {
        $this->make(1, 2);
        self::make(3, 4);
        static::make(5, 6);
        $other = new self('x');
        $other->make(7, 8);
        $this->named(first: 1, second: 2);
        $this->spread(...[1, 2]);
        $this->doc(
            1,
            2,
            3
        );
    }

    private function make($a, $b)
    {
        $b = 2;
        $c = function () use ($a) { return $a; };

        return [$a, $b, $c];
    }

    private function named($first, $second)
    {
        $first = 1;
        return [$first, $second];
    }

    private function spread($a, $b)
    {
        $a = 1;
        return $a + $b;
    }

    /**
     * Does things.
     *
     * @param int $x the x
     * @param int $y
     * @param int $z
     */
    private function doc($x, $y, $z)
    {
        $y = 5;
        $z = $y + 1;

        return $x + $y + $z;
    }

    private function untouched($a = 5, $b = null)
    {
        $a = $b;
        $b = 3;
        return [$a, $b];
    }

    private function deep($q)
    {
        if (true) {
            $q = 1;
        }
        $q = 2;
        return $q;
    }

    private function vv($q)
    {
        $q = 1;
        $name = 'q';
        return $$name;
    }

    private function str($q)
    {
        $q = "x";
        return "$q";
    }
}
-----
<?php

namespace Tests;

use PHPUnit\Framework\TestCase;

class SomeTest extends TestCase
{
    public function testA()
    {
        $this->make(1);
        self::make(3);
        static::make(5);
        $other = new self('x');
        $other->make(7);
        $this->named(first: 1, second: 2);
        $this->spread(...[1, 2]);
        $this->doc(
            1
        );
    }

    private function make($a)
    {
        $b = 2;
        $c = function () use ($a) { return $a; };

        return [$a, $b, $c];
    }

    private function named($first, $second)
    {
        $first = 1;
        return [$first, $second];
    }

    private function spread($a, $b)
    {
        $a = 1;
        return $a + $b;
    }

    /**
     * Does things.
     *
     * @param int $x the x
     */
    private function doc($x)
    {
        $y = 5;
        $z = $y + 1;

        return $x + $y + $z;
    }

    private function untouched($b = null)
    {
        $a = $b;
        $b = 3;
        return [$a, $b];
    }

    private function deep($q)
    {
        if (true) {
            $q = 1;
        }
        $q = 2;
        return $q;
    }

    private function vv($q)
    {
        $q = 1;
        $name = 'q';
        return $$name;
    }

    private function str()
    {
        $q = "x";
        return "$q";
    }
}
