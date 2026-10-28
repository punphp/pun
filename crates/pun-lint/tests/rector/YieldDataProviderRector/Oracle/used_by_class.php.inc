<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class UsedTest extends TestCase
{
    /**
     * @dataProvider p1
     * @dataProvider p2
     * @dataProvider p3
     * @dataProvider p4
     */
    public function test(int $a): void
    {
    }

    public static function p1()
    {
        return [[1]];
    }

    public function p2()
    {
        return [[1]];
    }

    public static function p3()
    {
        return [[1]];
    }

    public static function p4()
    {
        return [[1]];
    }

    public function testUses()
    {
        $this->p1();
        $this->p2();
        self::p3();
        static::p4();
    }
}
