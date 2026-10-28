<?php

namespace Pun\Vm;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class More2Test extends TestCase
{
    private MockObject $plain;
    private MockObject&Manager&Leaf $chain;
    private MockObject&Leaf&Base $chain2;

    protected function setUp(): void
    {
        $this->plain = 1;
        $this->plain = $this->other();
        $list[] = $this->createMock(Manager::class);
        $name = 'x';
        $$name = $this->createMock(Manager::class);
    }

    /**
     * @param object{a: int} $shape
     */
    public function testMore(object $shape, $builder): void
    {
        $shape->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->chain->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->chain2->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->plain->method('persist')->with('x');
        $this->plain->method('persist')->with($this->equalTo(1));
        $builder->with($this->callback(function ($e) {
            return true;
        }));
    }

    private function other()
    {
        return null;
    }
}

abstract class AbstractSetUpTest extends TestCase
{
    abstract protected function setUp(): void;

    public function testThat(): void
    {
        $this->createMock(Manager::class)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
    }
}
-----
<?php

namespace Pun\Vm;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class More2Test extends TestCase
{
    private MockObject $plain;
    private MockObject&Manager&Leaf $chain;
    private MockObject&Leaf&Base $chain2;

    protected function setUp(): void
    {
        $this->plain = 1;
        $this->plain = $this->other();
        $list[] = $this->createMock(Manager::class);
        $name = 'x';
        $$name = $this->createMock(Manager::class);
    }

    /**
     * @param object{a: int} $shape
     */
    public function testMore(object $shape, $builder): void
    {
        $shape->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->chain->method('persist')->willReturnCallback(function ($e): void {
        });
        $this->chain2->method('persist')->willReturnCallback(function ($e): void {
        });
        $this->plain->method('persist')->with('x');
        $this->plain->method('persist')->with($this->equalTo(1));
        $builder->with($this->callback(function ($e) {
            return true;
        }));
    }

    private function other()
    {
        return null;
    }
}

abstract class AbstractSetUpTest extends TestCase
{
    abstract protected function setUp(): void;

    public function testThat(): void
    {
        $this->createMock(Manager::class)->method('persist')->willReturnCallback(function ($e): void {
        });
    }
}
