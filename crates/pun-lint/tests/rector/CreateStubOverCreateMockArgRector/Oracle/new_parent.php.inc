<?php

namespace Pun\Ns4;

final class ParentTest extends Base
{
    public function testThat()
    {
        new parent($this->createMock(\stdClass::class));
        new Base($this->createMock(\stdClass::class));
        new \PHPUnit\Framework\TestCase($this->createMock(\stdClass::class));
    }
}

final class OtherTest extends \PHPUnit\Framework\TestCase
{
    public function testThat()
    {
        new parent($this->createMock(\stdClass::class));
    }
}
-----
<?php

namespace Pun\Ns4;

final class ParentTest extends Base
{
    public function testThat()
    {
        new parent($this->createMock(\stdClass::class));
        new Base($this->createStub(\stdClass::class));
        new \PHPUnit\Framework\TestCase($this->createStub(\stdClass::class));
    }
}

final class OtherTest extends \PHPUnit\Framework\TestCase
{
    public function testThat()
    {
        new parent($this->createMock(\stdClass::class));
    }
}
