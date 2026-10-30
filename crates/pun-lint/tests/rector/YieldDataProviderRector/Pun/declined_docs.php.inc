<?php

namespace Pun\Yd;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

final class TypesTest extends TestCase
{
    #[DataProvider('phpstanReturn')]
    #[DataProvider('intersection')]
    #[DataProvider('listOfUnion')]
    #[DataProvider('floatKey')]
    #[DataProvider('unionKey')]
    #[DataProvider('nestedFloatKey')]
    #[DataProvider('nestedListKey')]
    #[DataProvider('nestedUnion')]
    public function testIt($a)
    {
    }

    /**
     * @phpstan-return array<int>
     */
    public static function phpstanReturn()
    {
        return [[1]];
    }

    /**
     * @return Foo|Bar&Baz
     */
    public static function intersection()
    {
        return [[1]];
    }

    /**
     * @return (int|string)[]
     */
    public static function listOfUnion()
    {
        return [[1]];
    }

    /**
     * @return array<float, int>
     */
    public static function floatKey()
    {
        return [[1]];
    }

    /**
     * @return array<list<int>|int, int>
     */
    public static function unionKey()
    {
        return [[1]];
    }

    /**
     * @return array<int, array<float, int>>
     */
    public static function nestedFloatKey()
    {
        return [[1]];
    }

    /**
     * @return array<int, array<list<int>, int>>
     */
    public static function nestedListKey()
    {
        return [[1]];
    }

    /**
     * @return array<int, (int|string)|bool>
     */
    public static function nestedUnion()
    {
        return [[1]];
    }
}
