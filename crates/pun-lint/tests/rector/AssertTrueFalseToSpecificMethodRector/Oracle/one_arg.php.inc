<?php
namespace T;
use PHPUnit\Framework\TestCase;
final class ATest extends TestCase
{
    public function testIt($a): void
    {
        $this->assertTrue(in_array($a));
        $this->assertTrue(is_a());
    }
}
-----
<?php
namespace T;
use PHPUnit\Framework\TestCase;
final class ATest extends TestCase
{
    public function testIt($a): void
    {
        $this->assertContains($a);
        $this->assertTrue(is_a());
    }
}
