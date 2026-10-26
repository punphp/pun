<?php

use PHPUnit\Framework\TestCase;

final class GTest extends TestCase
{
    public function testMe()
    {
        self::assertSame(
            1,
            2
        );
        static::assertTrue(
            // why
            true,
        );
        SELF::assertTrue(...[true]);
        self::assertTrue(condition: true);
        self::assertCustom();
        self::fail('x');
        $f = function () { self::assertTrue(true); };
        $g = static function () { self::assertTrue(true); };
        self::AssertTrue(true);
        self::assertTrue(...);
        new class extends TestCase { public function t() { self::assertTrue(true); } };
    }

    public static function assertCustom() {}
}
-----
<?php

use PHPUnit\Framework\TestCase;

final class GTest extends TestCase
{
    public function testMe()
    {
        $this->assertSame(1, 2);
        $this->assertTrue(
            // why
            true
        );
        SELF::assertTrue(...[true]);
        $this->assertTrue(condition: true);
        self::assertCustom();
        self::fail('x');
        $f = function () { $this->assertTrue(true); };
        $g = static function () { self::assertTrue(true); };
        self::AssertTrue(true);
        self::assertTrue(...);
        new class extends TestCase { public function t() { self::assertTrue(true); } };
    }

    public static function assertCustom() {}
}
