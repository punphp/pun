<?php
class SomeTest extends \PHPUnit\Framework\TestCase
{
    public function testB()
    {
        $this->assertTrue(
            is_a($value, 'SomeClass'),
            'message'
        );
        $this->assertTrue(
            array_key_exists('a', $b),
            'message'
        );
        $this->assertTrue(array_key_exists('a',
            $b), 'message');
        $this->assertTrue(empty($x),
             'message');
        $this->assertFalse(is_null($x));
    }
}
-----
<?php
class SomeTest extends \PHPUnit\Framework\TestCase
{
    public function testB()
    {
        $this->assertInstanceOf(
            'SomeClass',
            $value,
            'message'
        );
        $this->assertArrayHasKey(
            'a',
            $b,
            'message'
        );
        $this->assertArrayHasKey('a', $b, 'message');
        $this->assertEmpty($x,
             'message');
        $this->assertNotNull($x);
    }
}
