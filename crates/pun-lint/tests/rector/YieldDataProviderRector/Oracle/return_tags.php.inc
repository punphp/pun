<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class DocTest extends TestCase
{
    /**
     * @dataProvider p1
     * @dataProvider p2
     * @dataProvider p3
     * @dataProvider p4
     * @dataProvider p5
     * @dataProvider p6
     * @dataProvider p7
     * @dataProvider p8
     */
    public function test(int $a): void
    {
    }

    /**
     * Provides data.
     *
     * @return mixed
     */
    public static function p1()
    {
        return [[1]];
    }

    /**
     * Provides data.
     *
     * @return array<int, string>   the data
     * @see Foo
     */
    public static function p2()
    {
        return [[1]];
    }

    /**
     * @return  array<int, string>
     */
    public static function p3()
    {
        return [[1]];
    }

    /**
     * @see Foo
     * @return iterable
     * @deprecated
     */
    public static function p4()
    {
        return [[1]];
    }

    /** @return iterable */
    public static function p5()
    {
        return [[1]];
    }

    /**
     * @return	array<int, string>
     */
    public static function p6()
    {
        return [[1]];
    }

    /**
     * @return iterable the data
     *   over two lines
     *
     * Text after.
     */
    public static function p7()
    {
        return [[1]];
    }

    /**
     * @return array<int, string> the data
     *   over two lines
     */
    public static function p8()
    {
        return [[1]];
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class DocTest extends TestCase
{
    /**
     * @dataProvider p1
     * @dataProvider p2
     * @dataProvider p3
     * @dataProvider p4
     * @dataProvider p5
     * @dataProvider p6
     * @dataProvider p7
     * @dataProvider p8
     */
    public function test(int $a): void
    {
    }

    /**
     * Provides data.
     */
    public static function p1(): \Iterator
    {
        yield [1];
    }

    /**
     * Provides data.
     *
     * @return \Iterator<int, string> the data
     * @see Foo
     */
    public static function p2(): \Iterator
    {
        yield [1];
    }

    /**
     * @return \Iterator<int, string>
     */
    public static function p3(): \Iterator
    {
        yield [1];
    }

    /**
     * @see Foo
     * @deprecated
     */
    public static function p4(): \Iterator
    {
        yield [1];
    }

    public static function p5(): \Iterator
    {
        yield [1];
    }

    /**
     * @return \Iterator<int, string>
     */
    public static function p6(): \Iterator
    {
        yield [1];
    }

    public static function p7(): \Iterator
    {
        yield [1];
    }

    /**
     * @return \Iterator<int, string> the data
     *   over two lines
     */
    public static function p8(): \Iterator
    {
        yield [1];
    }
}
