<?php

namespace App\Tests\IntersectionParam;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

abstract class IntersectionParamTest extends TestCase
{
    abstract public function testAbstract(): void;

    public function testCalls(): void
    {
        $count = 1;
        $count += 1;
        $mock = $this->createMock(\stdClass::class);
        $name = 'mock';
        $this->helper($mock, $$name);
        $mocks = [$mock];
        $this->helper(...$mocks);
    }

    private function helper(MockObject $first, MockObject $second): void
    {
    }
}
