<?php
namespace App;
use PHPUnit\Framework\TestCase;
class Foo {}
final class T extends TestCase
{
    public function test(?Foo $a)
    {
        self::assertNull($a);
        $this->assertNotNull($a);
    }
}
