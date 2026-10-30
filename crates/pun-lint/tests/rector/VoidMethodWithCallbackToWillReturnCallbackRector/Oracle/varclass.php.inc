<?php

namespace Pun\Vm;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class VarClassTest extends TestCase
{
    private MockObject $mock;

    protected function setUp(): void
    {
        $object = new Manager();
        $this->mock = $this->createMock($object::class);
    }

    public function testThat(): void
    {
        $this->mock->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
    }
}
