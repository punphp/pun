<?php

namespace App\Tests\MergeCallback;

use PHPUnit\Framework\TestCase;

final class MergeCallbackTest extends TestCase
{
    public function testIt($untyped): void
    {
        $mock = $this->createMock(\stdClass::class);
        $mock->willReturn(1);
        $mock->method('x')->willReturn(1);
        $mock->method('x')->with()->willReturn(1);
        $mock->method('x')->with(1)->willReturn(1);
        $mock->method('x')->with($this->other(1))->willReturn(1);
        $mock->method('x')->with($this->callback())->willReturn(1);
        $mock->method('x')->with($this->callback(fn ($x) => true))->willReturn(1);
        $mock->method('x')->with($this->callback(function ($x) {
        }))->willReturn(1);
        $mock->method('x')->with($this->callback(function ($x): bool {
            return true;
        }))->willReturn($untyped);
        $mock->method('x')->with($this->callback(function ($x) use ($mock): bool {
            return true;
        }))->willReturn($untyped);
        $mock->method('x')->with($this->callback(function ($x) {
            return true;
        }))->willReturn($untyped);
    }
}
-----
<?php

namespace App\Tests\MergeCallback;

use PHPUnit\Framework\TestCase;

final class MergeCallbackTest extends TestCase
{
    public function testIt($untyped): void
    {
        $mock = $this->createMock(\stdClass::class);
        $mock->willReturn(1);
        $mock->method('x')->willReturn(1);
        $mock->method('x')->with()->willReturn(1);
        $mock->method('x')->with(1)->willReturn(1);
        $mock->method('x')->with($this->other(1))->willReturn(1);
        $mock->method('x')->with($this->callback())->willReturn(1);
        $mock->method('x')->with($this->callback(fn ($x) => true))->willReturn(1);
        $mock->method('x')->with($this->callback(function ($x) {
        }))->willReturn(1);
        $mock->method('x')->willReturnCallback(function ($x) use ($untyped) {
            return $untyped;
        });
        $mock->method('x')->willReturnCallback(function ($x) use ($mock, $untyped) {
            return $untyped;
        });
        $mock->method('x')->willReturnCallback(function ($x) use ($untyped) {
            return $untyped;
        });
    }
}
