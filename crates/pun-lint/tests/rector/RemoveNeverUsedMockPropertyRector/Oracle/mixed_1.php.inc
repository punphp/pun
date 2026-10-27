<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

trait UsesOther
{
    public function other(): void
    {
        $this->traitUsed->foo();
    }
}

trait UsesNothing
{
}

final class SomeTest extends TestCase
{
    use UsesOther, UsesNothing, MissingTrait;

    private MockObject $a;
    private MockObject $b, $c;
    private MockObject $d;
    private MockObject $traitUsed;
    private static MockObject $e;
    private MockObject $f;
    private MockObject $g;

    protected function setUp(): void
    {
        $this->a = $this->createMock(\stdClass::class);
        $this->b = self::createMock(\stdClass::class);
        $this->d = $this->createMock(\stdClass::class);
        $this->traitUsed = $this->createMock(\stdClass::class);
        $this->f = $this->createMock(\stdClass::class);
        $this->g = $this->createStub(\stdClass::class);
        $this->a->expects($this->once())->method('x');
        $this->d?->foo();
        $this->b->method('x')->willReturn(1);
        $other = new \stdClass();
        $other->f = 1;
    }

    public function testSomething(): void
    {
        $this->a->method('y');
        $this->b->method('y');
        $value = $this->d->foo();
        $this->f->bar();
        $fn = function () {
            return $this->f;
        };
    }
}

final class SomeOtherTestCase extends TestCase
{
    private MockObject $a;

    protected function setUp(): void
    {
        $this->a = $this->createMock(\stdClass::class);
    }
}

final class NoSetUpTest extends TestCase
{
    private MockObject $a;

    protected function before(): void
    {
        $this->a = $this->createMock(\stdClass::class);
    }
}

final class OnlyPropertyTest extends TestCase
{
    private MockObject $a;

    protected function setUp(): void
    {
        $this->a = $this->createMock(\stdClass::class);
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

trait UsesOther
{
    public function other(): void
    {
        $this->traitUsed->foo();
    }
}

trait UsesNothing
{
}

final class SomeTest extends TestCase
{
    use UsesOther, UsesNothing, MissingTrait;
    private MockObject $d;
    private MockObject $traitUsed;
    private static MockObject $e;
    private MockObject $f;
    private MockObject $g;

    protected function setUp(): void
    {
        $this->d = $this->createMock(\stdClass::class);
        $this->traitUsed = $this->createMock(\stdClass::class);
        $this->f = $this->createMock(\stdClass::class);
        $this->g = $this->createStub(\stdClass::class);
        $this->d?->foo();
        $other = new \stdClass();
        $other->f = 1;
    }

    public function testSomething(): void
    {
        $value = $this->d->foo();
        $this->f->bar();
        $fn = function () {
            return $this->f;
        };
    }
}

final class SomeOtherTestCase extends TestCase
{
    private MockObject $a;

    protected function setUp(): void
    {
        $this->a = $this->createMock(\stdClass::class);
    }
}

final class NoSetUpTest extends TestCase
{
    private MockObject $a;

    protected function before(): void
    {
        $this->a = $this->createMock(\stdClass::class);
    }
}

final class OnlyPropertyTest extends TestCase
{
    protected function setUp(): void
    {
    }
}
