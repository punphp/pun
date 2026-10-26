<?php

use PHPUnit\Framework\TestCase;

final class GTest extends TestCase
{
    public function testMe()
    {
        SELF::assertTrue(true);
        self::assertTrue(...[true]);
        Static::assertTrue(true);
        self::assertTrue(true, ...$x);
    }
}

$x = new class extends TestCase { public function t() { self::assertTrue(true); } };
-----
<?php

use PHPUnit\Framework\TestCase;

final class GTest extends TestCase
{
    public function testMe()
    {
        SELF::assertTrue(true);
        $this->assertTrue(...[true]);
        Static::assertTrue(true);
        $this->assertTrue(true, ...$x);
    }
}

$x = new class extends TestCase { public function t() { self::assertTrue(true); } };
