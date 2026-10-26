<?php
class SomeTest extends \PHPUnit\Framework\TestCase
{
    public function testB()
    {
        $this->assertTrue(
            $a === $b,
            'message'
        );
        $this->assertTrue(
            $a === $b
        );
    }
}
-----
<?php
class SomeTest extends \PHPUnit\Framework\TestCase
{
    public function testB()
    {
        $this->assertSame(
            $b,
            $a,
            'message'
        );
        $this->assertSame($b, $a);
    }
}
