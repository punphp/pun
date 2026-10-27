<?php

namespace App\Tests\Cover8;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class Cover8Test extends TestCase
{
    public function testAsserts($a, $b, true|false $tf, MockObject|false $mf, MockObject|bool $mb): void
    {
        assert((bool) $a);
        assert(-$a);
        $this->assertFalse(strlen($a));
        $this->assertFalse(is_nan($a, $b));
        $this->assertTrue(
            in_array($a, $b),
            'message'
        );
        $expected = ['a' => 1];
        $this->assertEquals($expected, ['a' => 1]);
        $other = 'x';
        $this->assertEquals($other, 'x');
        $this->assertEquals(0x7FFFFFFFFFFFFFFFFF, $a);
        $this->assertEquals([Undefined::X => 1], [Undefined::X => 1]);
        $this->assertEquals('x', $tf);
        $this->assertStringContainsString('a', $a ? 'b' : 'c');
        $mf->method('x')->willReturnMap([[1, 2]]);
        $mb->method('x')->willReturnMap([[1, 2]]);
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->with('plain');
        $mock->method('x')->with(self::callback(fn ($x) => true));
        $mock->method('x')->with($this->callback(fn ($x) => $x === 1 && $x < 2));
        $mock->method('x')->with($this->callback(function ($x) use ($a) {
            return $x === $a && $x === 2;
        }));
        $mock->method('x')->with($this->callback(fn ($x) => count($x) === 0x7FFFFFFFFFFFFFFFFF));
    }

    public static function provide(): array
    {
        if (rand()) {
            return;
        }

        return [[1, 2], [3, 4]];
    }
}
