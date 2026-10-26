<?php
class SomeTest extends \PHPUnit\Framework\TestCase
{
    public function testB()
    {
        $this->assertFalse($strpos);
        $this->assertFalse($this->strpos);
    }
}
-----
<?php
class SomeTest extends \PHPUnit\Framework\TestCase
{
    public function testB()
    {
        $this->assertStringNotContainsString($strpos);
        $this->assertStringNotContainsString($this->strpos);
    }
}
