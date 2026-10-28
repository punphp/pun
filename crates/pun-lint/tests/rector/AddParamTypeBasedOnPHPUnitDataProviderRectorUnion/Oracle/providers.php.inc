<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

final class Item
{
}

final class ProbeTest extends TestCase
{
    public static function first(): array
    {
        return [
            'a' => ['x', new Item(), null, 1, true],
            'b' => ['y', [1, 2], 2, 1.5, false],
        ];
    }

    public static function second(): iterable
    {
        yield [3, null, 'z', 2, true];
        yield 'named' => [4, new Item(), 'w', 3, true];
    }

    public static function mixedOne(): array
    {
        return [[self::make()]];
    }

    public static function fromYield(): iterable
    {
        yield from self::first();
    }

    public static function make()
    {
        return 1;
    }

    #[DataProvider('first')]
    public function testFirst($a, $b, $c, $d, $e): void
    {
    }

    #[DataProvider('first')]
    #[DataProvider('second')]
    public function testBoth($a, $b, $c, $d, $e): void
    {
    }

    /**
     * @dataProvider second
     */
    public function testSecond($a, $b, $c, $d, $e): void
    {
    }

    #[DataProvider('mixedOne')]
    public function testMixed($a): void
    {
    }

    #[DataProvider('fromYield')]
    public function testFromYield($a): void
    {
    }

    #[DataProvider('first')]
    protected function testProtected($a): void
    {
    }
}
-----
<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

final class Item
{
}

final class ProbeTest extends TestCase
{
    public static function first(): array
    {
        return [
            'a' => ['x', new Item(), null, 1, true],
            'b' => ['y', [1, 2], 2, 1.5, false],
        ];
    }

    public static function second(): iterable
    {
        yield [3, null, 'z', 2, true];
        yield 'named' => [4, new Item(), 'w', 3, true];
    }

    public static function mixedOne(): array
    {
        return [[self::make()]];
    }

    public static function fromYield(): iterable
    {
        yield from self::first();
    }

    public static function make()
    {
        return 1;
    }

    #[DataProvider('first')]
    public function testFirst(string $a, \Probe\Item|array $b, ?int $c, int|float $d, bool $e): void
    {
    }

    #[DataProvider('first')]
    #[DataProvider('second')]
    public function testBoth(string|int $a, \Probe\Item|array|null $b, int|string|null $c, float|int $d, bool $e): void
    {
    }

    /**
     * @dataProvider second
     */
    public function testSecond($a, $b, $c, $d, $e): void
    {
    }

    #[DataProvider('mixedOne')]
    public function testMixed($a): void
    {
    }

    #[DataProvider('fromYield')]
    public function testFromYield($a): void
    {
    }

    #[DataProvider('first')]
    protected function testProtected($a): void
    {
    }
}
