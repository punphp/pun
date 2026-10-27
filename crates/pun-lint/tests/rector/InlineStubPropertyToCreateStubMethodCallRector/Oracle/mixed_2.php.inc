<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

abstract class SomeTest extends TestCase
{
    private Stub $a;
    private Stub $b;
    private Stub $cStub;

    protected function setUp(): void
    {
        $this->a = $this->createStub(\Countable::class);
        $this->b = $this->createStub(\Countable::class);
        $this->cStub = $this->createStub(\Countable::class);
        $this->a = $this->createStub(\ArrayAccess::class);
    }

    abstract public function helper(): void;

    public function testOne(): void
    {
        $obj?->foo($this->a);
        $x = new class($this->b) {};
        Foo::bar($this->a, $this->cStub);
        $y = 1;
        $cStub = 2;
        baz($this->cStub);
    }
}

final class BrokenTest extends TestCase
{
    private Stub $a;

    protected function setUp(): void
    {
        $this->a = $this->createStub($class);
    }

    public function testOne(): void
    {
        foo($this->a);
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

abstract class SomeTest extends TestCase
{
    private Stub $a;

    protected function setUp(): void
    {
        $this->a = $this->createStub(\Countable::class);
        $this->a = $this->createStub(\ArrayAccess::class);
    }

    abstract public function helper(): void;

    public function testOne(): void
    {
        $obj?->foo($this->a);
        $x = new class($this->createStub(\Countable::class)) {};
        $cStub = $this->createStub(\Countable::class);
        Foo::bar($this->a, $cStub);
        $y = 1;
        $cStub = 2;
        baz($cStub);
    }
}

final class BrokenTest extends TestCase
{
    private Stub $a;

    protected function setUp(): void
    {
        $this->a = $this->createStub($class);
    }

    public function testOne(): void
    {
        foo($this->a);
    }
}
