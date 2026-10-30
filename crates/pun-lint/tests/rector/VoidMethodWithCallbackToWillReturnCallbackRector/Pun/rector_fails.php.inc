<?php

namespace Pun\Vm;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class FailsTest extends TestCase
{
    private MockObject $mock;

    protected function setUp(): void
    {
        $this->mock = $this->createMock($this->make()::class);
    }

    private function make(): Manager
    {
        return new Manager();
    }

    public function testThat(): void
    {
        $this->mock->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
    }
}
