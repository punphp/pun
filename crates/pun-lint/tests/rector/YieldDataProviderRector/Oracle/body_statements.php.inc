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

    public static function p1(): array
    {
        return [];
    }

    public static function p2() : array
    {
        $a = [1];
        return ['x' => $a, 'y' => [2], ...$a, &$b];
    }

    public static function p3()
    {
        return [
            [1],
        ];
        $after = 1;
    }

    public static function p4()
    {
        yield from [[1], [2]];
        return [[3]];
    }

    public static function p5()
    {
        return array(
            array(
                'multi',
                'line',
            ),
            $x ? [1] : [2],
        ); // trailing
    }

    public static function p6()
    {
        return [[1]];
        // trailing comment
    }

    public static function p7()
    {
        $a = 1;
        $a .= 'x';
        $b = &$a;
        return [[$a]];
    }

    public static function p8()
    {
        $a = function () { return [1]; };
        return [[1]];
    }

    public static function p9()
    {
        return [[1]] ;
    }

    public function p10()
    {
        /* block */
        return [/* in */ [1], # hash
            [2]];
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

    public static function p1(): \Iterator
    {
    }

    public static function p2() : \Iterator
    {
        $a = [1];
        yield 'x' => $a;
        yield 'y' => [2];
        yield $a;
        yield $b;
    }

    public static function p3(): \Iterator
    {
        $after = 1;
        yield [1];
    }

    public static function p4(): \Iterator
    {
        yield [3];
    }

    public static function p5(): \Iterator
    {
        // trailing
        yield array(
            'multi',
            'line',
        );
        yield $x ? [1] : [2];
    }

    public static function p6(): \Iterator
    {
        // trailing comment
        yield [1];
    }

    public static function p7(): \Iterator
    {
        $a = 1;
        $a .= 'x';
        $b = &$a;
        yield [$a];
    }

    public static function p8(): \Iterator
    {
        $a = function () { return [1]; };
        yield [1];
    }

    public static function p9(): \Iterator
    {
        yield [1];
    }

    public function p10(): \Iterator
    {
        /* block */
        /* in */
        yield [1];
        # hash
        yield [2];
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
