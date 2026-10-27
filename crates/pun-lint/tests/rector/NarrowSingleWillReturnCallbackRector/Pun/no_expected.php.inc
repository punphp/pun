<?php

namespace App\Tests\NarrowCallback;

use PHPUnit\Framework\TestCase;

final class NarrowCallbackTest extends TestCase
{
    public function test($value)
    {
        $matcher = $this->exactly(3);
        $mock = $this->createMock(\stdClass::class);
        $mock->expects($matcher)->method('f')->willReturnCallback(function (...$parameters) use ($matcher) {
            match ($matcher->getInvocationCount()) {
                1 => $this->assertSame(),
            };
        });
    }
}
