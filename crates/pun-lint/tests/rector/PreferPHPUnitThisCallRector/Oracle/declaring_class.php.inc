<?php

final class LegacyTest extends \PHPUnit_Framework_TestCase
{
    public function testMe()
    {
        self::assertTrue(true);
    }
}

final class OverridingTest extends \PHPUnit\Framework\TestCase
{
    public function testMe()
    {
        self::assertTrue(true);
        self::assertSame(1, 1);
        static::once();
    }

    public static function assertTrue($condition, string $message = ''): void
    {
    }
}

final class NotATest
{
    public function testMe()
    {
        self::assertTrue(true);
    }
}
-----
<?php

final class LegacyTest extends \PHPUnit_Framework_TestCase
{
    public function testMe()
    {
        self::assertTrue(true);
    }
}

final class OverridingTest extends \PHPUnit\Framework\TestCase
{
    public function testMe()
    {
        self::assertTrue(true);
        $this->assertSame(1, 1);
        $this->once();
    }

    public static function assertTrue($condition, string $message = ''): void
    {
    }
}

final class NotATest
{
    public function testMe()
    {
        self::assertTrue(true);
    }
}
