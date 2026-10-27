<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    private MockObject $a;
    private ?MockObject $b;
    private MockObject&\Countable $c;
    private \Countable $d;

    public function testIt(): void
    {
        $this->a->method('x')->willReturnMap([[1, 2], [3, 4], [5, 6]]);
        $this->b->method('x')->willReturnMap([]);
        $this->c->method('x')->willReturnMap([1]);
        $this->d->method('x')->willReturnMap([1]);
        ($this->a)->method('x')->willReturnMap([1, 2]);
        $this->a->method('x')->willReturnMap($map);
        $this->a->method('x')->willReturnMap();
        $this->a?->method('x')->willReturnMap([1]);
        $this->a->method('x')?->willReturnMap([1]);
        $this->createMock(\Countable::class)->method('count')->willReturnMap([1]);
        $fn = function () {
            $this->a->method('y')->willReturnMap([1, 2]);
        };
        $x = $this->a->method('z')->willReturnMap([1]);
    }

    /** @test */
    public function documented(): void
    {
        $this->a->method('x')->willReturnMap([1]);
    }

    public function setup(): void
    {
        $this->a->method('x')->willReturnMap([1]);
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    private MockObject $a;
    private ?MockObject $b;
    private MockObject&\Countable $c;
    private \Countable $d;

    public function testIt(): void
    {
        $this->a->expects($this->atLeast(3))->method('x')->willReturnMap([[1, 2], [3, 4], [5, 6]]);
        $this->b->expects($this->atLeast(0))->method('x')->willReturnMap([]);
        $this->c->expects($this->atLeast(1))->method('x')->willReturnMap([1]);
        $this->d->method('x')->willReturnMap([1]);
        ($this->a->expects($this->atLeast(2)))->method('x')->willReturnMap([1, 2]);
        $this->a->method('x')->willReturnMap($map);
        $this->a->method('x')->willReturnMap();
        $this->a?->method('x')->willReturnMap([1]);
        $this->a->method('x')?->willReturnMap([1]);
        $this->createMock(\Countable::class)->method('count')->willReturnMap([1]);
        $fn = function () {
            $this->a->expects($this->atLeast(2))->method('y')->willReturnMap([1, 2]);
        };
        $x = $this->a->method('z')->willReturnMap([1]);
    }

    /** @test */
    public function documented(): void
    {
        $this->a->expects($this->atLeast(1))->method('x')->willReturnMap([1]);
    }

    public function setup(): void
    {
        $this->a->method('x')->willReturnMap([1]);
    }
}
