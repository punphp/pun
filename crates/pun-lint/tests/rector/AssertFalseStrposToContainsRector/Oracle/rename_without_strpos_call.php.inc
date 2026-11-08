<?php
class SomeTest extends \PHPUnit\Framework\TestCase
{
    public function testB()
    {
        $this->assertFalse($strpos);
        $this->assertFalse($this->strpos);
    }
}
