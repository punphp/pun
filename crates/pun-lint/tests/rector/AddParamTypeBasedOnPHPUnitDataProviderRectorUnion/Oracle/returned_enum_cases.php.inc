<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

enum Suit
{
    case Hearts;
    case Spades;
    case Clubs;
}

final class EnumsTest extends TestCase
{
    #[DataProvider('one')]
    public function testOne($value)
    {
    }

    #[DataProvider('two')]
    public function testTwo($value)
    {
    }

    #[DataProvider('intFirst')]
    public function testIntFirst($value)
    {
    }

    #[DataProvider('withArray')]
    public function testWithArray($value)
    {
    }

    public static function one()
    {
        return [[Suit::Hearts], [1]];
    }

    public static function two()
    {
        return [[Suit::Hearts], [Suit::Spades], [1]];
    }

    public static function intFirst()
    {
        return [[1], [Suit::Hearts]];
    }

    public static function withArray()
    {
        return [[Suit::Hearts], [[1]], [1]];
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
    case Clubs;
}

final class EnumsTest extends TestCase
{
    #[DataProvider('one')]
    public function testOne(\Probe\Suit|int $value)
    {
    }

    #[DataProvider('two')]
    public function testTwo(\Probe\Suit|int $value)
    {
    }

    #[DataProvider('intFirst')]
    public function testIntFirst(int|\Probe\Suit $value)
    {
    }

    #[DataProvider('withArray')]
    public function testWithArray(\Probe\Suit|int|array $value)
    {
    }

    public static function one()
    {
        return [[Suit::Hearts], [1]];
    }

    public static function two()
    {
        return [[Suit::Hearts], [Suit::Spades], [1]];
    }

    public static function intFirst()
    {
        return [[1], [Suit::Hearts]];
    }

    public static function withArray()
    {
        return [[Suit::Hearts], [[1]], [1]];
    }
}
