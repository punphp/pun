<?php

namespace Pun\Tests;

use PHPUnit\Framework\TestCase;
use Pun\Entity;

final class RectorFailsTest extends TestCase
{
    public function testNoMethod()
    {
        $aMock = $this->createMock(Entity\Child::class);
        $aMock->expects($this->once());
    }

    public function testConstantName()
    {
        $aMock = $this->createMock(Entity\Child::class);
        $aMock->method(FOO)->willReturn(1);
    }

    public function testNoArgument()
    {
        $aMock = $this->createMock(Entity\Child::class);
        $aMock->method()->willReturn(1);
    }

    public function testFirstClassCallable()
    {
        $aMock = $this->createMock(Entity\Child::class);
        $aMock->method(...);
    }

    public function testCaseMismatch()
    {
        $aMock = $this->createMock(Entity\Child::class);
        $AMOCK = $this->createMock(Entity\Child::class);
        $AMOCK->method('getX')->willReturn(1);
        $aMock->method('getX')->willReturn($AMOCK);
    }

    public function testOtherSpelling()
    {
        $aMock = $this->createMock(Entity\Child::class);
        $AMOCK = $this->createMock(Entity\Plain::class);
    }
}
