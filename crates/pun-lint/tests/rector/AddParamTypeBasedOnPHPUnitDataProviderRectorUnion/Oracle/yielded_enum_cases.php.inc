<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

enum Suit
{
    case Hearts;
    case Spades;
}

final class EnumsTest extends TestCase
{
    #[DataProvider('yields')]
    public function testYields($value)
    {
    }

    #[DataProvider('withNull')]
    public function testWithNull($value)
    {
    }

    #[DataProvider('yieldsNull')]
    public function testYieldsNull($value)
    {
    }

    public static function yields()
    {
        yield [Suit::Hearts];
        yield [[1]];
        yield [1];
    }

    public static function withNull()
    {
        return [[Suit::Hearts], [[1]], [null], [1]];
    }

    public static function yieldsNull()
    {
        yield [Suit::Hearts];
        yield [null];
        yield [1];
    }
}
-----
<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

enum Suit
{
    case Hearts;
    case Spades;
}

final class EnumsTest extends TestCase
{
    #[DataProvider('yields')]
    public function testYields(int|array|\Probe\Suit $value)
    {
    }

    #[DataProvider('withNull')]
    public function testWithNull(\Probe\Suit|int|array|null $value)
    {
    }

    #[DataProvider('yieldsNull')]
    public function testYieldsNull(int|\Probe\Suit|null $value)
    {
    }

    public static function yields()
    {
        yield [Suit::Hearts];
        yield [[1]];
        yield [1];
    }

    public static function withNull()
    {
        return [[Suit::Hearts], [[1]], [null], [1]];
    }

    public static function yieldsNull()
    {
        yield [Suit::Hearts];
        yield [null];
        yield [1];
    }
}
