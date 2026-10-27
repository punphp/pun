<?php

namespace App\Tests\NoArgs;

use PHPUnit\Framework\TestCase;

final class NoArgsTest extends TestCase
{
    public function testIt($a): void
    {
        $this->assertTrue();
        $this->assertFalse();
        $this->assertNotTrue();
        $this->assertNotFalse();
        $this->assertNull();
        $this->assertEquals();
        $this->assertNotEquals();
        $this->assertNotSame();
        $this->assertInstanceOf();
        $this->assertContainsOnly();
        $this->assertNotContainsOnly();
        $this->assertIsType();
        $this->assertStringContainsString();
        $this->assertStringContainsString('a');
        $this->assertTrue(is_a());
        $this->assertTrue(is_a($a));
        $this->assertTrue(str_contains($a));
        $this->assertFalse(strpos());
        $this->assertFalse(strpos($a));
        $this->assertSame(1, count());
        $this->assertSame(\stdClass::class, get_class());
        $mock = $this->createMock(\stdClass::class);
        $mock->expects();
        $mock->method('x')->will();
        $mock->method('x')->with();
        $mock->method('x')->with($this->equalTo());
        $mock->method('x')->with($this->callback());
        $mock->method('x')->willReturnCallback();
        $mock->method('x')->withConsecutive();
        $mock->method('x')->will($this->returnValue());
        $mock->method('x')->will(new \PHPUnit\Framework\MockObject\Stub\ReturnValueMap());
        $mock->method('x')->will(new \stdClass([]));
        $mock->method('x')->will(new $a([]));
        $mock->method('x')->will($this->returnSelf());
        $mock->method('x')->will($this->other(1));
        $mock->method('x')->will(self::returnValue(1));
        $mock->method('x')->will(self::other(1));
        $this->getMockBuilder()->getMock();
    }
}
