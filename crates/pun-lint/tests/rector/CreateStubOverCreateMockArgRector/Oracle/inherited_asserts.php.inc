<?php

namespace Pun\Ns3;

final class SelfTest extends \PHPUnit\Framework\TestCase
{
    public function testThat()
    {
        self::assertTrue($this->createMock(\stdClass::class));
        $this->assertTrue($this->createMock(\stdClass::class));
        self::helper($this->createMock(\stdClass::class));
        $this->assertSame(1, $this->createMock(\stdClass::class));
        \PHPUnit\Framework\Assert::assertTrue($this->createMock(\stdClass::class));
        $this->expectsOutput($this->createMock(\stdClass::class));
    }

    private static function helper($m) {}
}
-----
<?php

namespace Pun\Ns3;

final class SelfTest extends \PHPUnit\Framework\TestCase
{
    public function testThat()
    {
        self::assertTrue($this->createStub(\stdClass::class));
        $this->assertTrue($this->createStub(\stdClass::class));
        self::helper($this->createStub(\stdClass::class));
        $this->assertSame(1, $this->createStub(\stdClass::class));
        \PHPUnit\Framework\Assert::assertTrue($this->createStub(\stdClass::class));
        $this->expectsOutput($this->createStub(\stdClass::class));
    }

    private static function helper($m) {}
}
