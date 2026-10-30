<?php

namespace Pun\Ns2;

final class AssignsTest extends BaseTest
{
    public function testAssignsPassed()
    {
        $helper = new Helper();
        $taken = $this->createMock(\stdClass::class);
        $helper->take($taken);
        $doc = $this->createMock(\stdClass::class);
        $helper->docOnly($doc);
        $untyped = $this->createMock(\stdClass::class);
        $helper->untyped($untyped);
        $named = $this->createMock(\stdClass::class);
        $helper->second(m: $named, a: 1);
        $unknown = $this->createMock(\stdClass::class);
        $helper->undefined($unknown);
    }

    public function testAnonymousNamed()
    {
        new class(m: $this->createMock(\stdClass::class), n: $this->createMock(\stdClass::class)) {
            public function __construct($m, ?\PHPUnit\Framework\MockObject\MockObject $n) {}
        };
    }
}
-----
<?php

namespace Pun\Ns2;

final class AssignsTest extends BaseTest
{
    public function testAssignsPassed()
    {
        $helper = new Helper();
        $taken = $this->createMock(\stdClass::class);
        $helper->take($taken);
        $doc = $this->createMock(\stdClass::class);
        $helper->docOnly($doc);
        $untyped = $this->createStub(\stdClass::class);
        $helper->untyped($untyped);
        $named = $this->createMock(\stdClass::class);
        $helper->second(m: $named, a: 1);
        $unknown = $this->createStub(\stdClass::class);
        $helper->undefined($unknown);
    }

    public function testAnonymousNamed()
    {
        new class(m: $this->createStub(\stdClass::class), n: $this->createStub(\stdClass::class)) {
            public function __construct($m, ?\PHPUnit\Framework\MockObject\MockObject $n) {}
        };
    }
}
