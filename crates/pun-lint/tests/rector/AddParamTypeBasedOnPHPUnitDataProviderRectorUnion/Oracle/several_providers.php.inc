<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

enum Suit
{
    case Hearts;
    case Spades;
}

class Base
{
}

final class Child extends Base
{
}

final class UnionsTest extends TestCase
{
    #[DataProvider('bases')]
    #[DataProvider('children')]
    public function testBaseFirst($value)
    {
    }

    #[DataProvider('children')]
    #[DataProvider('bases')]
    public function testChildFirst($value)
    {
    }

    #[DataProvider('suits')]
    public function testSuits($value)
    {
    }

    public static function bases()
    {
        return [[new Base()], [null]];
    }

    public static function children()
    {
        return [[new Child()], [[1]]];
    }

    public static function suits()
    {
        return [[Suit::Hearts], [Suit::Spades], [Suit::Hearts], [1]];
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

class Base
{
}

final class Child extends Base
{
}

final class UnionsTest extends TestCase
{
    #[DataProvider('bases')]
    #[DataProvider('children')]
    public function testBaseFirst(\Probe\Base|array|null $value)
    {
    }

    #[DataProvider('children')]
    #[DataProvider('bases')]
    public function testChildFirst(\Probe\Base|array|null $value)
    {
    }

    #[DataProvider('suits')]
    public function testSuits(\Probe\Suit|int $value)
    {
    }

    public static function bases()
    {
        return [[new Base()], [null]];
    }

    public static function children()
    {
        return [[new Child()], [[1]]];
    }

    public static function suits()
    {
        return [[Suit::Hearts], [Suit::Spades], [Suit::Hearts], [1]];
    }
}
