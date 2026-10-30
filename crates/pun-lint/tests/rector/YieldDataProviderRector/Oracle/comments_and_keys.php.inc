<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class WsTest extends TestCase
{
    /**
     * @dataProvider p1
     * @dataProvider p2
     * @dataProvider p3
     * @dataProvider p4
     * @dataProvider p5
     */
    public function test(int $a): void
    {
    }

    public static function p1()
    {
        $a = 1;

        $b = 2;
           /**
         * odd doc
            */
        $c = 3; $d = 4;

        return [[1], [2]];
    }

    public static function p2()
    {
        // only
        return [[1], [2]];
    }

    public static function p3()
    {
        return [[1], [2]];

        // trailing
    }

    public static function p4()
    {
        yield from [[1]];
        $a = 1;
    }

    public static function p5()
    {
        return [
            1 => 'a', // one
            2 => 'b',
            'k' . 'x' => fn() => 1,
            yield 1,
        ];
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class WsTest extends TestCase
{
    /**
     * @dataProvider p1
     * @dataProvider p2
     * @dataProvider p3
     * @dataProvider p4
     * @dataProvider p5
     */
    public function test(int $a): void
    {
    }

    public static function p1(): \Iterator
    {
        $a = 1;

        $b = 2;
           /**
         * odd doc
            */
        $c = 3; $d = 4;
        yield [1];
        yield [2];
    }

    public static function p2(): \Iterator
    {
        // only
        yield [1];
        yield [2];
    }

    public static function p3(): \Iterator
    {
        // trailing
        yield [1];
        yield [2];
    }

    public static function p4(): \Iterator
    {
        $a = 1;
        yield [1];
    }

    public static function p5(): \Iterator
    {
        yield 1 => 'a';
        // one
        yield 2 => 'b';
        yield 'k' . 'x' => fn() => 1;
        yield yield 1;
    }
}
