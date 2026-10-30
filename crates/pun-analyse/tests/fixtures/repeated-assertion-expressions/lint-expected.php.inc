<?php

namespace RepeatedAssertionExpressions\Lint;

use PHPUnit\Framework\TestCase;

final class NeverValue extends TestCase
{
    public function testRepeated($a): void
    {
        $this->assertSame('5', $a);
        $this->assertEquals('5', $a);
        $this->assertTrue($a === $this->value());
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->willReturnCallback(function ($x) {
            return $this->value();
        });
    }

    private function value(): int
    {
        return 1;
    }
}
