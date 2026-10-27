<?php

namespace App\Tests\Cover5;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class NotTestWithSetUp
{
    /**
     * @requires PHP 8.0
     */
    protected function setUp(): void
    {
        parent::setUp();
    }
}

final class Cover5Test extends TestCase
{
    private (MockObject&\stdClass)|null $dnf;

    private Stub|null $nullableStub;

    private (Stub&\stdClass)|null $dnfStub;

    private array $list = [];

    protected function setUp(): void
    {
        $this->dnf = $this->createMock(\stdClass::class);
        $this->nullableStub = $this->createStub(\stdClass::class);
        $this->dnfStub = $this->createStub(\stdClass::class);
        static::$shared = 1;
        $this->list[] = 1;
    }

    public function testCalls(string $string, $a, $b): void
    {
        $this->assertSame($a);
        $this->assertEquals($a);
        $this->assertNotSame($a);
        $this->assertStringContainsString($a);
        $this->assertThat($a, $this->isType('string', 1));
        $this->assertThat($a, $this->isType(other: 'x'));
        $this->assertThat($a, $this->isType('foo'));
        $this->assertContainsOnly('foo', $a);
        $this->assertEquals(5, $string);
        $this->assertSame(0x7FFFFFFFFFFFFFFFFF, $string);
        $this->assertSame(5, '5');
        $this->assertSame('5', 5);
        $this->assertSame(5, 1.5);
        $this->assertTrue(-$a);
        $this->assertFalse(strlen($a));
        $this->assertFalse(strpos($a));
        $this->assertSame(\stdClass::class, get_class($a), 'message');
        $this->assertNotSame(\stdClass::class, get_class($a));
        $this->assertTrue($a instanceof $b->c);
        $this->assertTrue((bool) $a);
        $this->assertSame('x', $a ? true : false);
        $this->assertEquals('x', null);
        $this->assertEquals('x', static::class);
        $this->assertEquals('x', (object) []);
        $this->assertEquals('x', $this->nothing());
        $builder = $this->getMockBuilder(\stdClass::class);
        $builder->getMock();
        $this->getMockBuilder(\stdClass::class)->disableOriginalConstructor()->getMock();
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->with($this->equalTo());
        $mock->method('x')->will($this->returnValue());
        $mock->method('x')->will(self::returnValue());
        $mock->method('x')->with($this->callback($a));
        $mock->method('x')->with($this->callback(function ($x) {
            $y = 1;
            return true;
        }));
        $mock->method('x')->with($this->callback(fn ($x) => $x instanceof $b));
        $mock->method('x')->with($this->callback(fn ($x) => $x == 1));
        $mock->method('x')->with($this->callback(fn ($x) => $x instanceof self));
        $mock->method('x')->with($this->callback(fn ($x) => $x instanceof \stdClass));
        $mock->method('x')->with($this->callback(fn ($x) => $x === 1 && $x->y()));
        $mock->method('x')->with($this->callback(function ($x) use ($a) {
            return $x === $a && $x->y();
        }));
    }

    public function testForeach($items): void
    {
        foreach ($items as $item) {
            if ($item) {
            }
        }
        foreach ($items as $item) {
            $this->assertSame(1, $item);
        }
        foreach ($items as $item) {
            $this->assertInstanceOf(\stdClass::class);
        }
    }

    private function nothing(): void
    {
    }
}
-----
<?php

namespace App\Tests\Cover5;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class NotTestWithSetUp
{
    /**
     * @requires PHP 8.0
     */
    protected function setUp(): void
    {
        parent::setUp();
    }
}

final class Cover5Test extends TestCase
{
    private (MockObject&\stdClass)|null $dnf;

    private Stub|null $nullableStub;

    private (Stub&\stdClass)|null $dnfStub;

    private array $list = [];

    protected function setUp(): void
    {
        $this->dnf = $this->createMock(\stdClass::class);
        $this->nullableStub = $this->createStub(\stdClass::class);
        $this->dnfStub = $this->createStub(\stdClass::class);
        static::$shared = 1;
        $this->list[] = 1;
    }

    public function testCalls(string $string, $a, $b): void
    {
        $this->assertSame($a);
        $this->assertEquals($a);
        $this->assertNotSame($a);
        $this->assertStringContainsString($a);
        $this->assertThat($a, $this->isType('string', 1));
        $this->assertThat($a, $this->isType(other: 'x'));
        $this->assertThat($a, $this->isType('foo'));
        $this->assertContainsOnly('foo', $a);
        $this->assertEquals(5, $string);
        $this->assertSame(0x7FFFFFFFFFFFFFFFFF, $string);
        $this->assertSame(5, '5');
        $this->assertSame('5', 5);
        $this->assertSame(5, 1.5);
        $this->assertTrue(-$a);
        $this->assertFalse(strlen($a));
        $this->assertFalse(strpos($a));
        $this->assertSame(\stdClass::class, get_class($a), 'message');
        $this->assertNotSame(\stdClass::class, get_class($a));
        $this->assertTrue($a instanceof $b->c);
        $this->assertTrue((bool) $a);
        $this->assertSame('x', $a ? true : false);
        $this->assertEquals('x', null);
        $this->assertEquals('x', static::class);
        $this->assertEquals('x', (object) []);
        $this->assertEquals('x', $this->nothing());
        $builder = $this->getMockBuilder(\stdClass::class);
        $builder->getMock();
        $this->createMock(\stdClass::class);
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->with($this->equalTo());
        $mock->method('x')->will($this->returnValue());
        $mock->method('x')->will(self::returnValue());
        $mock->method('x')->with($this->callback($a));
        $mock->method('x')->with($this->callback(function ($x) {
            $y = 1;
            return true;
        }));
        $mock->method('x')->with($this->callback(fn ($x) => $x instanceof $b));
        $mock->method('x')->with($this->callback(fn ($x) => $x == 1));
        $mock->method('x')->with($this->callback(fn ($x) => $x instanceof self));
        $mock->method('x')->with($this->callback(fn ($x) => $x instanceof \stdClass));
        $mock->method('x')->with($this->callback(fn ($x) => $x === 1 && $x->y()));
        $mock->method('x')->with($this->callback(function ($x) use ($a) {
            return $x === $a && $x->y();
        }));
    }

    public function testForeach($items): void
    {
        foreach ($items as $item) {
            if ($item) {
            }
        }
        foreach ($items as $item) {
            $this->assertSame(1, $item);
        }
        foreach ($items as $item) {
            $this->assertInstanceOf(\stdClass::class);
        }
    }

    private function nothing(): void
    {
    }
}
