<?php

namespace Pun\Ns;

final class RoundsTest extends BaseTest
{
    public function testLastAssign()
    {
        $a = $this->createStub(\stdClass::class);
        $a = $this->createStub(\DateTime::class);
        $this->parentTake($A);
    }
}
-----
<?php

namespace Pun\Ns;

final class RoundsTest extends BaseTest
{
    public function testLastAssign()
    {
        $a = $this->createMock(\stdClass::class);
        $a = $this->createMock(\DateTime::class);
        $this->parentTake($A);
    }
}
