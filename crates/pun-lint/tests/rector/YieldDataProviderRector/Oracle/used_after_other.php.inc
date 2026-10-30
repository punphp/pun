<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class BodyTest extends TestCase
{
    /**
     * @dataProvider p1
     * @dataProvider p2()
     * @dataProvider p3
     * @dataProvider p4
     * @dataProvider p5
     * @dataProvider P6
     * @dataProvider p7
     * @dataProvider p8
     * @dataProvider p9
     * @dataProvider p10
     * @dataProvider p11
     */
    public function test(int $a): void
    {
    }

    public static function p9()
    {
        return [[1]] ;
    }

    public static function p11()
    {
        return [[1]];
    }

    public static function notAProvider()
    {
        return [[1]];
    }

    public function testUsesP11()
    {
        $this->p11();
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class BodyTest extends TestCase
{
    /**
     * @dataProvider p1
     * @dataProvider p2()
     * @dataProvider p3
     * @dataProvider p4
     * @dataProvider p5
     * @dataProvider P6
     * @dataProvider p7
     * @dataProvider p8
     * @dataProvider p9
     * @dataProvider p10
     * @dataProvider p11
     */
    public function test(int $a): void
    {
    }

    public static function p9(): \Iterator
    {
        yield [1];
    }

    public static function p11()
    {
        return [[1]];
    }

    public static function notAProvider()
    {
        return [[1]];
    }

    public function testUsesP11()
    {
        $this->p11();
    }
}
