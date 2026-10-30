<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;
use Some\Other as Foo;

final class DocsTest extends TestCase
{
    #[\PHPUnit\Framework\Attributes\DataProvider('p0')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p1')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p2')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p3')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p4')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p5')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p6')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p7')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p8')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p9')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p10')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p11')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p12')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p13')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p14')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p15')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p16')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p17')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p18')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p19')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p20')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p21')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p22')]
    public function test(int $a): void
    {
    }

    /** @return array<int, true|false> */
    public static function p0()
    {
        return [[1]];
    }

    /** @return array<int, mixed|null> */
    public static function p1()
    {
        return [[1]];
    }

    /** @return array|array */
    public static function p2()
    {
        return [[1]];
    }

    /** @return array|string[] */
    public static function p3()
    {
        return [[1]];
    }

    /** @return array<int, string>|array<string, int> */
    public static function p4()
    {
        return [[1]];
    }

    /** @return array<int<0, max>, int> */
    public static function p5()
    {
        return [[1]];
    }

    /** @return array <int, string> */
    public static function p6()
    {
        return [[1]];
    }

    /** @return array<int, string>&Countable */
    public static function p7()
    {
        return [[1]];
    }

    /** @return array<int, string|object> */
    public static function p8()
    {
        return [[1]];
    }

    /** @return array<int, array<int>|array<string>> */
    public static function p9()
    {
        return [[1]];
    }

    /** @return array<int, mixed|int> */
    public static function p10()
    {
        return [[1]];
    }

    /** @return array<int, array-key|null> */
    public static function p11()
    {
        return [[1]];
    }

    /** @return array<int, resource|int> */
    public static function p12()
    {
        return [[1]];
    }

    /** @return array<int, void|null> */
    public static function p13()
    {
        return [[1]];
    }

    /** @return array{a: int} */
    public static function p14()
    {
        return [[1]];
    }

    /** @return array<int, array{0: string, 1: int}> */
    public static function p15()
    {
        return [[1]];
    }

    /** @return array<string, array{string, int}> */
    public static function p16()
    {
        return [[1]];
    }

    /** @return array<int, 1> */
    public static function p17()
    {
        return [[1]];
    }

    /** @return array<int, 'a'> */
    public static function p18()
    {
        return [[1]];
    }

    /** @return array<int, int<0, 5>> */
    public static function p19()
    {
        return [[1]];
    }

    /** @return array<int, \Closure(): void> */
    public static function p20()
    {
        return [[1]];
    }

    /** @return array<int, \Iterator<int>> */
    public static function p21()
    {
        return [[1]];
    }

    /** @return array<int, namespace\Foo> */
    public static function p22()
    {
        return [[1]];
    }
}
