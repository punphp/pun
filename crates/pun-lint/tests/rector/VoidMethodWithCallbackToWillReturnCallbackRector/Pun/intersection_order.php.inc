<?php

namespace Pun\Vm;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class OrderTest extends TestCase
{
    private MockObject&Manager&Returns $first;
    private MockObject&Returns&Manager $second;
    private Returns&MockObject&Manager $third;

    public function testOrder(): void
    {
        $this->first->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->second->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->third->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
    }
}
