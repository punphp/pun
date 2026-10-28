<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

class Item
{
    public static function make(): static
    {
        return new static();
    }
}

final class KindsTest extends TestCase
{
    private $fixture = 'value';

    #[DataProvider('legacyProvider')]
    public function testLegacy($value)
    {
    }

    public static function legacyProvider()
    {
        return array(array(1), array(2));
    }

    #[DataProvider('variableProvider')]
    public function testVariable($value)
    {
    }

    public static function variableProvider()
    {
        $set = [1];

        return [$set];
    }

    #[DataProvider('shortProvider')]
    public function testShort($first, $second)
    {
    }

    public static function shortProvider()
    {
        return [[1], [2, 'a']];
    }

    #[DataProvider('keyedYields')]
    public function testKeyedYields($first, $second)
    {
    }

    public static function keyedYields()
    {
        yield ['first' => 1, 'second' => 'a'];
        yield ['first' => 2, 'second' => 'b'];
    }

    #[DataProvider('scalarYield')]
    public function testScalarYield($value)
    {
    }

    public static function scalarYield()
    {
        yield 1;
    }

    #[DataProvider('staticProvider')]
    public function testStatic($value)
    {
    }

    public static function staticProvider()
    {
        return [[new static('a')]];
    }

    #[DataProvider('madeProvider')]
    public function testMade($value)
    {
    }

    public static function madeProvider()
    {
        return [[Item::make()]];
    }
}
-----
<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

class Item
{
    public static function make(): static
    {
        return new static();
    }
}

final class KindsTest extends TestCase
{
    private $fixture = 'value';

    #[DataProvider('legacyProvider')]
    public function testLegacy(int $value)
    {
    }

    public static function legacyProvider()
    {
        return array(array(1), array(2));
    }

    #[DataProvider('variableProvider')]
    public function testVariable($value)
    {
    }

    public static function variableProvider()
    {
        $set = [1];

        return [$set];
    }

    #[DataProvider('shortProvider')]
    public function testShort(int $first, string $second)
    {
    }

    public static function shortProvider()
    {
        return [[1], [2, 'a']];
    }

    #[DataProvider('keyedYields')]
    public function testKeyedYields(int $first, string $second)
    {
    }

    public static function keyedYields()
    {
        yield ['first' => 1, 'second' => 'a'];
        yield ['first' => 2, 'second' => 'b'];
    }

    #[DataProvider('scalarYield')]
    public function testScalarYield($value)
    {
    }

    public static function scalarYield()
    {
        yield 1;
    }

    #[DataProvider('staticProvider')]
    public function testStatic($value)
    {
    }

    public static function staticProvider()
    {
        return [[new static('a')]];
    }

    #[DataProvider('madeProvider')]
    public function testMade(\Probe\Item $value)
    {
    }

    public static function madeProvider()
    {
        return [[Item::make()]];
    }
}
