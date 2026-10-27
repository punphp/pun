<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class Helper
{
    public function take(MockObject&\stdClass $first, $second = null)
    {
    }

    public function either(MockObject|Stub $first)
    {
    }
}

final class SomeTest extends TestCase
{
    public function testHelper(): void
    {
        $helper = new Helper();
        $a = $this->createMock(\stdClass::class);
        $helper->take($a);
        $b = $this->createMock(\stdClass::class);
        $helper->either($b);
        $d = $this->createMock(\stdClass::class);
        $helper->take(second: $d, first: 1);
        $e = $this->createMock(\stdClass::class);
        $helper->missing($e);
        $f = $this->createMock();
        $g = $this->createMock(\stdClass::class);
        $h = $this->createMock(\stdClass::class);
        (new Helper())->take($h);
        $$name = $this->createMock(\stdClass::class);
        $i = $this->createMock(\stdClass::class);
        $j = $this->createMock(\stdClass::class);
        $fn = function () use ($i) { $i->method('x'); };
        $arrow = fn () => $j->method('x');
        $k = $this->createMock(\stdClass::class);
        $this->{$name}($k);
    }

    /**
     * @test
     */
    public function documentedTest(): void
    {
        $a = $this->createMock(\stdClass::class);
        $a = 1;
    }

    #[\PHPUnit\Framework\Attributes\Test]
    public function attributed(): void
    {
        $a = $this->createMock(\stdClass::class);
        foo($a);
    }

    public function notATest(): void
    {
        $a = $this->createMock(\stdClass::class);
        foo($a);
    }

    protected function testProtected(): void
    {
        $a = $this->createMock(\stdClass::class);
        foo($a);
    }

    public function testShort(): void
    {
        $a = $this->createMock(\stdClass::class);
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class Helper
{
    public function take(MockObject&\stdClass $first, $second = null)
    {
    }

    public function either(MockObject|Stub $first)
    {
    }
}

final class SomeTest extends TestCase
{
    public function testHelper(): void
    {
        $helper = new Helper();
        $a = $this->createMock(\stdClass::class);
        $helper->take($a);
        $b = $this->createStub(\stdClass::class);
        $helper->either($b);
        $d = $this->createMock(\stdClass::class);
        $helper->take(second: $d, first: 1);
        $e = $this->createStub(\stdClass::class);
        $helper->missing($e);
        $f = $this->createMock();
        $g = $this->createStub(\stdClass::class);
        $h = $this->createStub(\stdClass::class);
        (new Helper())->take($h);
        $$name = $this->createMock(\stdClass::class);
        $i = $this->createStub(\stdClass::class);
        $j = $this->createMock(\stdClass::class);
        $fn = function () use ($i) { $i->method('x'); };
        $arrow = fn () => $j->method('x');
        $k = $this->createStub(\stdClass::class);
        $this->{$name}($k);
    }

    /**
     * @test
     */
    public function documentedTest(): void
    {
        $a = $this->createStub(\stdClass::class);
        $a = 1;
    }

    #[\PHPUnit\Framework\Attributes\Test]
    public function attributed(): void
    {
        $a = $this->createStub(\stdClass::class);
        foo($a);
    }

    public function notATest(): void
    {
        $a = $this->createMock(\stdClass::class);
        foo($a);
    }

    protected function testProtected(): void
    {
        $a = $this->createMock(\stdClass::class);
        foo($a);
    }

    public function testShort(): void
    {
        $a = $this->createMock(\stdClass::class);
    }
}
