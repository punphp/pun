<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

final class Box
{
}

final class YieldTest extends TestCase
{
    #[DataProvider('assignedProvider')]
    public function testAssigned($given, $expected)
    {
    }

    public static function assignedProvider()
    {
        yield [1, 2];
        yield [$box = new Box(), $box];
    }

    #[DataProvider('plainProvider')]
    public function testPlain($given, $expected, $extra)
    {
    }

    public static function plainProvider()
    {
        $other = new Box();
        yield [new Box(), $other];
        yield [new Box(), new Box()];
    }
}
-----
<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

final class Box
{
}

final class YieldTest extends TestCase
{
    #[DataProvider('assignedProvider')]
    public function testAssigned(int|\Probe\Box $given, $expected)
    {
    }

    public static function assignedProvider()
    {
        yield [1, 2];
        yield [$box = new Box(), $box];
    }

    #[DataProvider('plainProvider')]
    public function testPlain(\Probe\Box $given, \Probe\Box $expected, $extra)
    {
    }

    public static function plainProvider()
    {
        $other = new Box();
        yield [new Box(), $other];
        yield [new Box(), new Box()];
    }
}
