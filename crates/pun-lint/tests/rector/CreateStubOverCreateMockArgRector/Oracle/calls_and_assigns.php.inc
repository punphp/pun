<?php

namespace Pun\Ns2;

use PHPUnit\Framework\MockObject\MockObject;

final class CasesTest extends BaseTest
{
    private MockObject $prop;
    private static $shared;

    public function testCalls()
    {
        $helper = new Helper();
        $helper->take($this->createMock(\stdClass::class));
        $helper->docOnly($this->createMock(\stdClass::class));
        $helper->nullable($this->createMock(\stdClass::class));
        $helper->untyped($this->createMock(\stdClass::class));
        $helper->stub($this->createMock(\stdClass::class));
        $helper->intersection($this->createMock(\stdClass::class));
        $helper->variadic($this->createMock(\stdClass::class), $this->createMock(\stdClass::class));
        $helper->second(1, $this->createMock(\stdClass::class));
        $helper->second(m: $this->createMock(\stdClass::class), a: 1);
        $helper->second(1, other: $this->createMock(\stdClass::class));
        $helper->untyped(($this->createMock(\stdClass::class)));
        $helper->untyped($this->CreateMock(\stdClass::class));
        $helper->untyped($helper->createMock(\stdClass::class));
        $helper->generic($this->createMock(\stdClass::class));
        $helper->magicMock($this->createMock(\stdClass::class));
        $helper->magicAny($this->createMock(\stdClass::class));
        $helper->undefined($this->createMock(\stdClass::class));
        $helper->untyped([$this->createMock(\stdClass::class)]);
        $helper->untyped($this->createMock(\stdClass::class)->method('a'));
        Helper::takeStatic($this->createMock(\stdClass::class));
        Unknown::run($this->createMock(\stdClass::class));
        $unknown->run($this->createMock(\stdClass::class));
        $this->parentTake($this->createMock(\stdClass::class));
        $this->parentAny($this->createMock(\stdClass::class));
        parent::parentAny($this->createMock(\stdClass::class));
        self::parentAny($this->createMock(\stdClass::class));
        static::parentTake($this->createMock(\stdClass::class));
        (new Other())->take($this->createMock(\stdClass::class));
        $mock = $this->createMock(Other::class);
        $mock->take($this->createMock(\stdClass::class));
        $fn = function () use ($helper) {
            $helper->untyped($this->createMock(\stdClass::class));
        };
    }

    public function testNew()
    {
        new Helper($this->createMock(\stdClass::class), $this->createMock(\stdClass::class));
        new MockHolder($this->createMock(\stdClass::class));
        new NoConstructor($this->createMock(\stdClass::class));
        new Unknown($this->createMock(\stdClass::class));
        new parent($this->createMock(\stdClass::class));
        new self($this->createMock(\stdClass::class));
        $class = Helper::class;
        new $class($this->createMock(\stdClass::class));
        $object = new Helper();
        new $object($this->createMock(\stdClass::class));
        new class($this->createMock(\stdClass::class)) {
            public function __construct($m) {}
        };
    }

    public function testArrays()
    {
        $items = ['key' => $this->createMock(\stdClass::class), $this->createMock(\stdClass::class)];
        $more = [...$this->createMock(\stdClass::class)];
        $nested = [[$this->createMock(\stdClass::class)], $this->createStub(\stdClass::class)];
    }

    public function testAssigns()
    {
        $unused = $this->createMock(\stdClass::class);
        $mocked = $this->createMock(\stdClass::class);
        $mocked->method('a');
        $taken = $this->createMock(\stdClass::class);
        (new Helper())->take($taken);
        $docTaken = $this->createMock(\stdClass::class);
        (new Helper())->docOnly($docTaken);
        $named = $this->createMock(\stdClass::class);
        (new Helper())->second(m: $named, a: 1);
        $given = $this->createMock(\stdClass::class);
        (new Helper())->untyped($given);
        $staticTaken = $this->createMock(\stdClass::class);
        Helper::takeStatic($staticTaken);
        $noArgs = $this->createMock();
        $this->prop = $this->createMock(\stdClass::class);
        self::$shared = $this->createMock(\stdClass::class);
        $list['x'] = $this->createMock(\stdClass::class);
        $name = 'dyn';
        $$name = $this->createMock(\stdClass::class);
        $Case = $this->createMock(\stdClass::class);
        $case->method('a');
        if (true) {
            $nestedAssign = $this->createMock(\stdClass::class);
        }
        $closureUse = $this->createMock(\stdClass::class);
        $fn = function () use ($closureUse) {
            $closureUse->method('a');
        };
    }

    /** @test */
    public function annotated()
    {
        $unused = $this->createMock(\stdClass::class);
    }

    private function helperMethod()
    {
        $unused = $this->createMock(\stdClass::class);
    }

    protected function testProtected()
    {
        $unused = $this->createMock(\stdClass::class);
    }
}

final class NotATest
{
    public function testCalls()
    {
        $unused = $this->createMock(\stdClass::class);
        (new Helper())->untyped($this->createMock(\stdClass::class));
        $items = [$this->createMock(\stdClass::class)];
    }
}
-----
<?php

namespace Pun\Ns2;

use PHPUnit\Framework\MockObject\MockObject;

final class CasesTest extends BaseTest
{
    private MockObject $prop;
    private static $shared;

    public function testCalls()
    {
        $helper = new Helper();
        $helper->take($this->createMock(\stdClass::class));
        $helper->docOnly($this->createMock(\stdClass::class));
        $helper->nullable($this->createStub(\stdClass::class));
        $helper->untyped($this->createStub(\stdClass::class));
        $helper->stub($this->createStub(\stdClass::class));
        $helper->intersection($this->createMock(\stdClass::class));
        $helper->variadic($this->createMock(\stdClass::class), $this->createStub(\stdClass::class));
        $helper->second(1, $this->createMock(\stdClass::class));
        $helper->second(m: $this->createMock(\stdClass::class), a: 1);
        $helper->second(1, other: $this->createStub(\stdClass::class));
        $helper->untyped(($this->createStub(\stdClass::class)));
        $helper->untyped($this->createStub(\stdClass::class));
        $helper->untyped($helper->createStub(\stdClass::class));
        $helper->generic($this->createMock(\stdClass::class));
        $helper->magicMock($this->createMock(\stdClass::class));
        $helper->magicAny($this->createStub(\stdClass::class));
        $helper->undefined($this->createMock(\stdClass::class));
        $helper->untyped([$this->createStub(\stdClass::class)]);
        $helper->untyped($this->createMock(\stdClass::class)->method('a'));
        Helper::takeStatic($this->createMock(\stdClass::class));
        Unknown::run($this->createMock(\stdClass::class));
        $unknown->run($this->createMock(\stdClass::class));
        $this->parentTake($this->createMock(\stdClass::class));
        $this->parentAny($this->createStub(\stdClass::class));
        parent::parentAny($this->createStub(\stdClass::class));
        self::parentAny($this->createStub(\stdClass::class));
        static::parentTake($this->createMock(\stdClass::class));
        (new Other())->take($this->createMock(\stdClass::class));
        $mock = $this->createMock(Other::class);
        $mock->take($this->createMock(\stdClass::class));
        $fn = function () use ($helper) {
            $helper->untyped($this->createStub(\stdClass::class));
        };
    }

    public function testNew()
    {
        new Helper($this->createStub(\stdClass::class), $this->createStub(\stdClass::class));
        new MockHolder($this->createMock(\stdClass::class));
        new NoConstructor($this->createMock(\stdClass::class));
        new Unknown($this->createMock(\stdClass::class));
        new parent($this->createMock(\stdClass::class));
        new self($this->createStub(\stdClass::class));
        $class = Helper::class;
        new $class($this->createMock(\stdClass::class));
        $object = new Helper();
        new $object($this->createStub(\stdClass::class));
        new class($this->createStub(\stdClass::class)) {
            public function __construct($m) {}
        };
    }

    public function testArrays()
    {
        $items = ['key' => $this->createStub(\stdClass::class), $this->createStub(\stdClass::class)];
        $more = [...$this->createStub(\stdClass::class)];
        $nested = [[$this->createStub(\stdClass::class)], $this->createStub(\stdClass::class)];
    }

    public function testAssigns()
    {
        $unused = $this->createStub(\stdClass::class);
        $mocked = $this->createMock(\stdClass::class);
        $mocked->method('a');
        $taken = $this->createStub(\stdClass::class);
        (new Helper())->take($taken);
        $docTaken = $this->createStub(\stdClass::class);
        (new Helper())->docOnly($docTaken);
        $named = $this->createStub(\stdClass::class);
        (new Helper())->second(m: $named, a: 1);
        $given = $this->createStub(\stdClass::class);
        (new Helper())->untyped($given);
        $staticTaken = $this->createStub(\stdClass::class);
        Helper::takeStatic($staticTaken);
        $noArgs = $this->createStub();
        $this->prop = $this->createMock(\stdClass::class);
        self::$shared = $this->createStub(\stdClass::class);
        $list['x'] = $this->createStub(\stdClass::class);
        $name = 'dyn';
        $$name = $this->createMock(\stdClass::class);
        $Case = $this->createMock(\stdClass::class);
        $case->method('a');
        if (true) {
            $nestedAssign = $this->createMock(\stdClass::class);
        }
        $closureUse = $this->createStub(\stdClass::class);
        $fn = function () use ($closureUse) {
            $closureUse->method('a');
        };
    }

    /** @test */
    public function annotated()
    {
        $unused = $this->createStub(\stdClass::class);
    }

    private function helperMethod()
    {
        $unused = $this->createMock(\stdClass::class);
    }

    protected function testProtected()
    {
        $unused = $this->createMock(\stdClass::class);
    }
}

final class NotATest
{
    public function testCalls()
    {
        $unused = $this->createMock(\stdClass::class);
        (new Helper())->untyped($this->createMock(\stdClass::class));
        $items = [$this->createMock(\stdClass::class)];
    }
}
