<?php

namespace App\Tests;


use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @dataProvider provideA()
     * @dataProvider PROVIDEB
     * @dataProvider provideMissing
     * @dataProvider Other::provideC
     */
    #[\PHPUnit\Framework\Attributes\DataProvider('provideD')]
    #[\PHPUnit\Framework\Attributes\DataProvider(methodName: 'provideE')]
    #[\Codeception\Attribute\DataProvider('provideF')]
    public function testSomething($v)
    {
    }

    /** @dataProvider provideA */
    public function testTwice($v) {}

    public function provideA()
    {
        yield [1];
    }

    public function provideB()
    {
        $f = function () { yield 1; };
        return [];
    }

    public function provideC()
    {
        yield from [[1]];
    }

    public static function provideD()
    {
        yield 'a' => [1];
    }

    public function provideE()
    {
        if (true) {
            yield [2];
        }
    }

    public function provideF()
    {
        yield [3];
    }
}
-----
<?php

namespace App\Tests;


use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @dataProvider provideA()
     * @dataProvider PROVIDEB
     * @dataProvider provideMissing
     * @dataProvider Other::provideC
     */
    #[\PHPUnit\Framework\Attributes\DataProvider('provideD')]
    #[\PHPUnit\Framework\Attributes\DataProvider(methodName: 'provideE')]
    #[\Codeception\Attribute\DataProvider('provideF')]
    public function testSomething($v)
    {
    }

    /** @dataProvider provideA */
    public function testTwice($v) {}

    public function provideA(): \Iterator
    {
        yield [1];
    }

    public function provideB()
    {
        $f = function () { yield 1; };
        return [];
    }

    public function provideC()
    {
        yield from [[1]];
    }

    public static function provideD(): \Iterator
    {
        yield 'a' => [1];
    }

    public function provideE(): \Iterator
    {
        if (true) {
            yield [2];
        }
    }

    public function provideF(): \Iterator
    {
        yield [3];
    }
}
