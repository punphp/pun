<?php

namespace App\Tests\CreateStub;

use PHPUnit\Framework\TestCase;

final class CreateStubTest extends TestCase
{
    private $first, $second;

    public $hooked {
        get => $this->hooked;
        set => $value;
    }

    protected function setUp(): void
    {
        $this->first = $this->createMock(\stdClass::class);
        $this->second = $this->createMock(\stdClass::class);
        $this->hooked = $this->createMock(\stdClass::class);
        $this->plain = new \stdClass();
        $this->{'dynamic'} = $this->createMock(\stdClass::class);
    }
}

abstract class AbstractSetUpTest extends TestCase
{
    private $mock;

    abstract protected function setUp(): void;
}

function anonymous()
{
    return new class extends TestCase {
        private $mock;

        protected function setUp(): void
        {
            $this->mock = $this->createMock(\stdClass::class);
        }
    };
}
-----
<?php

namespace App\Tests\CreateStub;

use PHPUnit\Framework\TestCase;

final class CreateStubTest extends TestCase
{
    private \PHPUnit\Framework\MockObject\Stub $first, $second;

    public \PHPUnit\Framework\MockObject\Stub $hooked {
        get => $this->hooked;
        set => $value;
    }

    protected function setUp(): void
    {
        $this->first = $this->createStub(\stdClass::class);
        $this->second = $this->createStub(\stdClass::class);
        $this->hooked = $this->createStub(\stdClass::class);
        $this->plain = new \stdClass();
        $this->{'dynamic'} = $this->createMock(\stdClass::class);
    }
}

abstract class AbstractSetUpTest extends TestCase
{
    private $mock;

    abstract protected function setUp(): void;
}

function anonymous()
{
    return new class extends TestCase {
        private \PHPUnit\Framework\MockObject\Stub $mock;

        protected function setUp(): void
        {
            $this->mock = $this->createStub(\stdClass::class);
        }
    };
}
