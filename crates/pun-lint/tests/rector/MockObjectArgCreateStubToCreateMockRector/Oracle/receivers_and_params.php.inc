<?php

namespace Pun\Ns;

use PHPUnit\Framework\MockObject\MockObject;

final class CasesTest extends BaseTest
{
    public function testStatic()
    {
        $a = $this->createStub(\stdClass::class);
        Helper::takeStatic($a);
    }

    public function testOtherObject()
    {
        $a = $this->createStub(\stdClass::class);
        $helper = new Helper();
        $helper->take($a);
    }

    public function testDocOnly()
    {
        $a = $this->createStub(\stdClass::class);
        (new Helper())->docOnly($a);
    }

    public function testNullable()
    {
        $a = $this->createStub(\stdClass::class);
        (new Helper())->nullable($a);
    }

    public function testUntyped()
    {
        $a = $this->createStub(\stdClass::class);
        (new Helper())->untyped($a);
    }

    public function testStubParam()
    {
        $a = $this->createStub(\stdClass::class);
        (new Helper())->stub($a);
    }

    public function testVariadic()
    {
        $a = $this->createStub(\stdClass::class);
        $b = $this->createStub(\stdClass::class);
        (new Helper())->variadic($b, $a);
    }

    public function testSecondNamed()
    {
        $a = $this->createStub(\stdClass::class);
        (new Helper())->second(m: $a, a: 1);
        (new Helper())->second(a: 1, x: $a);
    }

    public function testNew()
    {
        $a = $this->createStub(\stdClass::class);
        new Helper($a);
    }

    public function testParent()
    {
        $a = $this->createStub(\stdClass::class);
        parent::parentTake($a);
        $b = $this->createStub(\stdClass::class);
        self::parentTake($b);
        $c = $this->createStub(\stdClass::class);
        static::parentTake($c);
    }

    public function testClosure()
    {
        $a = $this->createStub(\stdClass::class);
        $f = function () use ($a) { $this->parentTake($a); };
        $g = fn() => $this->parentTake($a);
    }

    public function testIntersection(MockObject&Helper $h)
    {
        $a = $this->createStub(\stdClass::class);
        $h->take($a);
    }

    public function testUnion(\DateTime|Helper $h)
    {
        $a = $this->createStub(\stdClass::class);
        $h->take($a);
    }

    public function testNested()
    {
        $a = $this->createStub(\stdClass::class);
        $this->foo($this->parentTake($a));
        $b = $other->createStub(\stdClass::class);
        $this->parentTake(...$b);
        $c = $this->createStub(\stdClass::class);
    }
}
-----
<?php

namespace Pun\Ns;

use PHPUnit\Framework\MockObject\MockObject;

final class CasesTest extends BaseTest
{
    public function testStatic()
    {
        $a = $this->createMock(\stdClass::class);
        Helper::takeStatic($a);
    }

    public function testOtherObject()
    {
        $a = $this->createMock(\stdClass::class);
        $helper = new Helper();
        $helper->take($a);
    }

    public function testDocOnly()
    {
        $a = $this->createMock(\stdClass::class);
        (new Helper())->docOnly($a);
    }

    public function testNullable()
    {
        $a = $this->createStub(\stdClass::class);
        (new Helper())->nullable($a);
    }

    public function testUntyped()
    {
        $a = $this->createStub(\stdClass::class);
        (new Helper())->untyped($a);
    }

    public function testStubParam()
    {
        $a = $this->createStub(\stdClass::class);
        (new Helper())->stub($a);
    }

    public function testVariadic()
    {
        $a = $this->createStub(\stdClass::class);
        $b = $this->createMock(\stdClass::class);
        (new Helper())->variadic($b, $a);
    }

    public function testSecondNamed()
    {
        $a = $this->createMock(\stdClass::class);
        (new Helper())->second(m: $a, a: 1);
        (new Helper())->second(a: 1, x: $a);
    }

    public function testNew()
    {
        $a = $this->createStub(\stdClass::class);
        new Helper($a);
    }

    public function testParent()
    {
        $a = $this->createMock(\stdClass::class);
        parent::parentTake($a);
        $b = $this->createMock(\stdClass::class);
        self::parentTake($b);
        $c = $this->createMock(\stdClass::class);
        static::parentTake($c);
    }

    public function testClosure()
    {
        $a = $this->createMock(\stdClass::class);
        $f = function () use ($a) { $this->parentTake($a); };
        $g = fn() => $this->parentTake($a);
    }

    public function testIntersection(MockObject&Helper $h)
    {
        $a = $this->createStub(\stdClass::class);
        $h->take($a);
    }

    public function testUnion(\DateTime|Helper $h)
    {
        $a = $this->createStub(\stdClass::class);
        $h->take($a);
    }

    public function testNested()
    {
        $a = $this->createMock(\stdClass::class);
        $this->foo($this->parentTake($a));
        $b = $other->createMock(\stdClass::class);
        $this->parentTake(...$b);
        $c = $this->createStub(\stdClass::class);
    }
}
