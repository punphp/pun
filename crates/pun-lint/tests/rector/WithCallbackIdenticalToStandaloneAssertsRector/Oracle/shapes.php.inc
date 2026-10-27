<?php

namespace App\Tests\Standalone;

use PHPUnit\Framework\TestCase;

final class StandaloneTest extends TestCase
{
    public function testIt($y, $fn): void
    {
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->with($this->callback('is_int'));
        $mock->method('x')->with($this->callback(fn ($x) => $x === 1 && ($x || $y)));
        $mock->method('x')->with($this->callback(fn ($x) => $fn('a', $x)));
        $mock->method('x')->with($this->callback(fn ($x) => $x === 1 && $y));
        $mock->method('x')->with($this->callback(function &($x) {
            return $x === 1;
        }));
        $mock->method('x')->with($this->callback(function ($x) use ($y) {
            return $x === $y;
        }));
        $mock->method('x')->with($this->callback(function ($x) {
            return $x === 1;
            // a comment
        }));
        $mock->method('x')->with($this->callback(fn ($x) => $x === new class($y) {
            public function __construct($value)
            {
            }
        }));
        $mock->method('x')->with($this->callback(fn ($x) => $x === (function () use ($y) {
            return $y;
        })()));
    }
}
-----
<?php

namespace App\Tests\Standalone;

use PHPUnit\Framework\TestCase;

final class StandaloneTest extends TestCase
{
    public function testIt($y, $fn): void
    {
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->with($this->callback('is_int'));
        $mock->method('x')->with($this->callback(fn ($x) => $x === 1 && ($x || $y)));
        $mock->method('x')->with($this->callback(fn ($x) => $fn('a', $x)));
        $mock->method('x')->with($this->callback(fn ($x) => $x === 1 && $y));
        $mock->method('x')->with($this->callback(function &($x): void {
            $this->assertSame(1, $x);
            return;
        }));
        $mock->method('x')->with($this->callback(function ($x) use ($y): void {
            $this->assertSame($y, $x);
            return;
        }));
        $mock->method('x')->with($this->callback(function ($x): void {
            // a comment
            $this->assertSame(1, $x);
            return;
        }));
        $mock->method('x')->with($this->callback(function ($x) use ($y): void {
            $this->assertSame(new class($y) {
                public function __construct($value)
                {
                }
            }, $x);
            return;
        }));
        $mock->method('x')->with($this->callback(function ($x): void {
            $this->assertSame((function () use ($y) {
                return $y;
            })(), $x);
            return;
        }));
    }
}
