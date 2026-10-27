<?php

namespace App\Tests\NarrowCallback;

use PHPUnit\Framework\TestCase;

final class NarrowCallbackTest extends TestCase
{
    public function test($value)
    {
        $matcher = $this->exactly(3);
        $mock = $this->createMock(\stdClass::class);
        $mock->expects($matcher)->method('a')->willReturnCallback(function (...$parameters) use ($matcher) {
            match ($matcher->getInvocationCount()) {
                1 => $this->assertSame([1], $parameters),
            };
            return 5;
        });
        $mock->expects($matcher)->method('c')->willReturnCallback(function (...$parameters) use ($matcher) {
            match ($matcher->getInvocationCount()) {
                1 => assertSame([1], $parameters),
            };
        });
        $mock->expects($matcher)->method('d')->willReturnCallback(function (...$parameters) use ($matcher) {
            match ($matcher->getInvocationCount()) {
                1 => self::assertSame(array(1, 2), $parameters),
            };
        });
        $mock->expects($matcher)->method('e')->willReturnCallback(function (...$parameters) use ($matcher) {
            match ($matcher->getInvocationCount()) {
                1 => $this->assertSame([1, ...$value], $parameters),
            };
        });
        $mock->expects($matcher)->method('b')->willReturnCallback(function (...$parameters) use ($matcher) {
            $value;
        });
        $mock->expects($matcher)->method('g')->willReturnCallback(function (...$parameters) use ($matcher) {
            match ($matcher->getInvocationCount()) {
                1 => $this->assertSame([1, 'k' => 2], $parameters),
            };
            return match ($matcher->getInvocationCount()) {
                1 => 'one',
            };
        });
    }
}
-----
<?php

namespace App\Tests\NarrowCallback;

use PHPUnit\Framework\TestCase;

final class NarrowCallbackTest extends TestCase
{
    public function test($value)
    {
        $matcher = $this->exactly(3);
        $mock = $this->createMock(\stdClass::class);
        $mock->expects($matcher)->method('a')->willReturnCallback(function (...$parameters) use ($matcher) {
            match ($matcher->getInvocationCount()) {
                1 => $this->assertSame([1], $parameters),
            };
            return 5;
        });
        $mock->expects($matcher)->method('c')->willReturnCallback(function (...$parameters) use ($matcher) {
            match ($matcher->getInvocationCount()) {
                1 => assertSame([1], $parameters),
            };
        });
        $mock->expects($matcher)->method('d')->with(1, 2);
        $mock->expects($matcher)->method('e')->with(1, $value);
        $mock->expects($matcher)->method('b')->willReturnCallback(function (...$parameters) use ($matcher) {
            $value;
        });
        $mock->expects($matcher)->method('g')->with(1, 2)->willReturn('one');
    }
}
