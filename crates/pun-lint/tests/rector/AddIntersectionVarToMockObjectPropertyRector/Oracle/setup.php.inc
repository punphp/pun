<?php

namespace App\Tests\IntersectionVar;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class IntersectionVarTest extends TestCase
{
    private MockObject $first, $second;

    private Stub $stub, $otherStub;

    private MockObject $assignedTwice;

    public \stdClass $hooked {
        get => $this->hooked;
    }

    protected function setUp(): void
    {
        $this->undeclared = $this->createMock(\stdClass::class);
        $this->first = $this->createMock(\stdClass::class);
        $this->second = $this->createMock(\DateTime::class);
        $this->stub = $this->createStub(\stdClass::class);
        $this->otherStub = $this->createStub(\DateTime::class);
        $this->assignedTwice = $this->createMock(\stdClass::class);
        $this->assignedTwice = $this->createMock(\DateTime::class);
        $this->{'dynamic'} = $this->createMock(\stdClass::class);
        $this->hooked = new \stdClass();
        $this->first = new \stdClass();
        self::$static = $this->createMock(\stdClass::class);
        $local = $this->createMock(\stdClass::class);
        $this->stub = self::createStub(\stdClass::class);
    }
}

abstract class AbstractSetUpTest extends TestCase
{
    private MockObject $mock;

    abstract protected function setUp(): void;
}
-----
<?php

namespace App\Tests\IntersectionVar;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class IntersectionVarTest extends TestCase
{
    /**
     * @var \PHPUnit\Framework\MockObject\MockObject&\stdClass
     */
    private MockObject $first, $second;

    private Stub $stub, $otherStub;

    /**
     * @var \PHPUnit\Framework\MockObject\MockObject&\DateTime
     */
    private MockObject $assignedTwice;

    public \stdClass $hooked {
        get => $this->hooked;
    }

    protected function setUp(): void
    {
        $this->undeclared = $this->createMock(\stdClass::class);
        $this->first = $this->createMock(\stdClass::class);
        $this->second = $this->createMock(\DateTime::class);
        $this->stub = $this->createStub(\stdClass::class);
        $this->otherStub = $this->createStub(\DateTime::class);
        $this->assignedTwice = $this->createMock(\stdClass::class);
        $this->assignedTwice = $this->createMock(\DateTime::class);
        $this->{'dynamic'} = $this->createMock(\stdClass::class);
        $this->hooked = new \stdClass();
        $this->first = new \stdClass();
        self::$static = $this->createMock(\stdClass::class);
        $local = $this->createMock(\stdClass::class);
        $this->stub = self::createStub(\stdClass::class);
    }
}

abstract class AbstractSetUpTest extends TestCase
{
    private MockObject $mock;

    abstract protected function setUp(): void;
}
