<?php

namespace Pun\Ns;

abstract class EdgesTest extends BaseTest
{
    abstract public function testAbstract();

    public function testDynamicName()
    {
        $a = $this->createStub(\stdClass::class);
        $this->$name($a);
    }

    public function testStaticOnObject()
    {
        $a = $this->createStub(\stdClass::class);
        $helper = new Helper();
        $helper::takeStatic($a);
    }
}

final class NotATestCase
{
    public function testNoTest()
    {
        $a = $this->createStub(\stdClass::class);
        Helper::takeStatic($a);
    }
}
-----
<?php

namespace Pun\Ns;

abstract class EdgesTest extends BaseTest
{
    abstract public function testAbstract();

    public function testDynamicName()
    {
        $a = $this->createStub(\stdClass::class);
        $this->$name($a);
    }

    public function testStaticOnObject()
    {
        $a = $this->createMock(\stdClass::class);
        $helper = new Helper();
        $helper::takeStatic($a);
    }
}

final class NotATestCase
{
    public function testNoTest()
    {
        $a = $this->createStub(\stdClass::class);
        Helper::takeStatic($a);
    }
}
