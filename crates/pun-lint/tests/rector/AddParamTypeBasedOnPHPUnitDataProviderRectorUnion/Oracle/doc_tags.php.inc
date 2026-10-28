<?php

namespace Probe;

use PHPUnit\Framework\TestCase;

final class Item
{
}

final class ProbeTest extends TestCase
{
    public static function second(): iterable
    {
        yield [3, null, 'z', 2, true];
        yield 'named' => [4, new Item(), 'w', 3, true];
    }

    public static function third(): iterable
    {
        yield [3, null, 'z', 2, true];
        yield 'named' => [4, 5, 'w', 3, true];
    }

    /**
     * @dataProvider second
     */
    public function testSecond($a, $b, $c, $d, $e): void
    {
    }

    /**
     * @dataProvider third
     */
    public function testThird($a, $b, $c, $d, $e): void
    {
    }
}
-----
<?php

namespace Probe;

use PHPUnit\Framework\TestCase;

final class Item
{
}

final class ProbeTest extends TestCase
{
    public static function second(): iterable
    {
        yield [3, null, 'z', 2, true];
        yield 'named' => [4, new Item(), 'w', 3, true];
    }

    public static function third(): iterable
    {
        yield [3, null, 'z', 2, true];
        yield 'named' => [4, 5, 'w', 3, true];
    }

    /**
     * @dataProvider second
     */
    public function testSecond(int $a, ?\Probe\Item $b, string $c, int $d, bool $e): void
    {
    }

    /**
     * @dataProvider third
     */
    public function testThird(int $a, ?int $b, string $c, int $d, bool $e): void
    {
    }
}
