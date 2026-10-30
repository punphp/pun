<?php

namespace Pun\Ns3;

final class SelfTest extends \PHPUnit\Framework\TestCase
{
    public function testThat()
    {
        new self($this->createMock(\stdClass::class));
        new static($this->createMock(\stdClass::class));
        new parent($this->createMock(\stdClass::class));
        new Plain($this->createMock(\stdClass::class));
        $x = new self('a');
        $x->expectsOutput($this->createMock(\stdClass::class));
        self::assertTrue($this->createMock(\stdClass::class));
    }
}
-----
<?php

namespace Pun\Ns3;

final class SelfTest extends \PHPUnit\Framework\TestCase
{
    public function testThat()
    {
        new self($this->createStub(\stdClass::class));
        new static($this->createStub(\stdClass::class));
        new parent($this->createMock(\stdClass::class));
        new Plain($this->createStub(\stdClass::class));
        $x = new self('a');
        $x->expectsOutput($this->createStub(\stdClass::class));
        self::assertTrue($this->createStub(\stdClass::class));
    }
}
