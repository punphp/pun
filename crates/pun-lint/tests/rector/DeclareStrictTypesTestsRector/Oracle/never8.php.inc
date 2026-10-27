<?php

namespace App\Tests\Never8;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class Never8Test extends TestCase
{
    public function testA()
    {
        $this->fail('x');
        return $this->value();
    }

    public function testAsserts($a): void
    {
        $this->fail('x');
        $this->assertSame('5', $a);
        $this->assertEquals('5', $a);
        $this->assertTrue($a === $this->value());
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->with($this->callback(function ($x) {
            return true;
        }))->willReturn($this->value());
    }

    private function mocked(): MockObject
    {
        $this->fail('x');
        return $this->createMock(\stdClass::class);
    }

    private function value(): int
    {
        return 1;
    }
}
-----
<?php

declare(strict_types=1);

namespace App\Tests\Never8;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class Never8Test extends TestCase
{
    public function testA()
    {
        $this->fail('x');
        return $this->value();
    }

    public function testAsserts($a): void
    {
        $this->fail('x');
        $this->assertSame('5', $a);
        $this->assertEquals('5', $a);
        $this->assertTrue($a === $this->value());
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->with($this->callback(function ($x) {
            return true;
        }))->willReturn($this->value());
    }

    private function mocked(): MockObject
    {
        $this->fail('x');
        return $this->createMock(\stdClass::class);
    }

    private function value(): int
    {
        return 1;
    }
}
