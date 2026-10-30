<?php

namespace Pun\Ns3;

final class ProbeTest extends \PHPUnit\Framework\TestCase
{
    public function testThat()
    {
        Probe::mixedParam($this->createMock(\stdClass::class));
        Probe::finalStatic($this->createMock(\stdClass::class));
        Probe::asserted($this->createMock(\stdClass::class));
        Probe::assertedMixed($this->createMock(\stdClass::class));
        Probe::voidRet($this->createMock(\stdClass::class));
        Probe::voidRet($this->createMock(\stdClass::class));
    }
}
-----
<?php

namespace Pun\Ns3;

final class ProbeTest extends \PHPUnit\Framework\TestCase
{
    public function testThat()
    {
        Probe::mixedParam($this->createStub(\stdClass::class));
        Probe::finalStatic($this->createStub(\stdClass::class));
        Probe::asserted($this->createStub(\stdClass::class));
        Probe::assertedMixed($this->createStub(\stdClass::class));
        Probe::voidRet($this->createStub(\stdClass::class));
        Probe::voidRet($this->createStub(\stdClass::class));
    }
}
