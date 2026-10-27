<?php

namespace App\Tests\MergeCallback;

use PHPUnit\Framework\TestCase;

final class MergeCallbackTest extends TestCase
{
    public function testIt($untyped): void
    {
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->with($this->callback(function ($x) {
            return true;
        }))->willReturn();
    }
}
