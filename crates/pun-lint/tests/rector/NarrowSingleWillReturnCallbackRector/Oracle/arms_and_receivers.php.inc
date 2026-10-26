<?php

final class R2Test extends \PHPUnit\Framework\TestCase
{
    public function test()
    {
        $mock->expects($m)->willReturnCallback(function (...$p) use ($m) {
            match ($m->numberOfInvocations()) {
                1 => self::assertEquals(['k' => 'a', ...$rest, (2)], $p),
            };
        });
        $mock->expects($m)->willReturnCallback(
            function (...$p) use ($m) {
                match ($m->numberOfInvocations()) {
                    0x1 => $this->assertSame('a', $p),
                };
                return match (true) {
                    2 => 'first', default => 'x',
                };
            }
        );
        $mock->expects($m)->willReturnCallback(function (...$p) use ($m) {
            match ($m->numberOfInvocations()) {
                1 => $other->assertSame('a', $p),
            };
        });
        $mock->expects($m)->willReturnCallback(function (...$p) use ($m) {
            match ($m->numberOfInvocations()) {
                1, 2 => $this->assertSame('a', $p),
            };
        });
    }
}
-----
<?php

final class R2Test extends \PHPUnit\Framework\TestCase
{
    public function test()
    {
        $mock->expects($m)->with('a', $rest, 2);
        $mock->expects($m)->with(
            'a'
        )->willReturn('first');
        $mock->expects($m)->willReturnCallback(function (...$p) use ($m) {
            match ($m->numberOfInvocations()) {
                1 => $other->assertSame('a', $p),
            };
        });
        $mock->expects($m)->willReturnCallback(function (...$p) use ($m) {
            match ($m->numberOfInvocations()) {
                1, 2 => $this->assertSame('a', $p),
            };
        });
    }
}
