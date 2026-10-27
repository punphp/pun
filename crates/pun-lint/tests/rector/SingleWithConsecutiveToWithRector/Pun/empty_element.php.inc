<?php

namespace App\Tests\Consecutive;

use PHPUnit\Framework\TestCase;

final class ConsecutivePunTest extends TestCase
{
    public function testIt($a): void
    {
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->withConsecutive([1, , $this->equalTo(1)]);
    }
}
