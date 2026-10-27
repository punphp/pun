<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

// Rector reads `@param MockObject` and keeps $a, and makes $b a stub; Pun sees no native type on either parameter
// and keeps both.
final class SomeTest extends TestCase
{
    public function test(): void
    {
        $a = $this->createMock(\stdClass::class);
        $this->documented($a);
        $b = $this->createMock(\stdClass::class);
        $this->untyped($b);
    }

    /** @param MockObject $mock */
    private function documented($mock)
    {
        $mock->expects($this->once())->method('x');
    }

    private function untyped($value)
    {
    }
}
