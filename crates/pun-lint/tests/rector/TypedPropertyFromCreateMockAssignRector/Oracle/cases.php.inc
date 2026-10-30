<?php

namespace App;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class MockTest extends TestCase
{
    private $a, $b;
    private $private;
    private $withDefault = 5;
    private $later = false;
    private \DateTime $wrong;
    private MockObject|\DateTime $union;
    private ?MockObject $nullable;
    private $notMock;
    private $mixed;
    private $dynamic;
    private $stub;

    protected function setUp(): void
    {
        $this->a = $this->createMock(\DateTime::class);
        $this->private = $this->createMock(\DateTime::class);
        $this->withDefault = $this->createMock(\DateTime::class);
        $this->wrong = $this->createMock(\DateTime::class);
        $this->union = $this->createMock(\DateTime::class);
        $this->nullable = $this->createMock(\DateTime::class);
        $this->notMock = new \DateTime();
        $this->mixed = $this->createMock(\DateTime::class);
        $this->mixed = new \DateTime();
        $this->stub = $this->createStub(\DateTime::class);
    }

    public function testLater(): void
    {
        $this->later = $this->createMock(\DateTime::class);
        $this->dynamic = $this->createMock(\DateTime::class);
    }
}

final class DynamicTest extends TestCase
{
    private $mock;

    protected function setUp(): void
    {
        $this->mock = $this->createMock(\DateTime::class);
        $name = 'other';
        $this->$name = 1;
    }
}

final class NotATest
{
    private $mock;

    protected function setUp(): void
    {
        $this->mock = $this->createMock(\DateTime::class);
    }
}

final class Mock84Test extends TestCase
{
    private $intersection;
    private $defaultNull = null;
    private $resetLater = null;
    private $hooked { get => $this->hooked; }

    protected function setUp(): void
    {
        $this->intersection = $this->make();
        $this->hooked = $this->createMock(\DateTime::class);
    }

    public function testIt(): void
    {
        $this->defaultNull = $this->createMock(\DateTime::class);
        $this->resetLater = $this->createMock(\DateTime::class);
    }

    protected function tearDown(): void
    {
        $this->resetLater = null;
    }

    private function make(): MockObject&\DateTime
    {
        return $this->createMock(\DateTime::class);
    }
}
-----
<?php

namespace App;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class MockTest extends TestCase
{
    private $a, $b;
    private \PHPUnit\Framework\MockObject\MockObject $private;
    private $withDefault = 5;
    private $later = false;
    private \PHPUnit\Framework\MockObject\MockObject $wrong;
    private \PHPUnit\Framework\MockObject\MockObject $union;
    private ?MockObject $nullable;
    private $notMock;
    private $mixed;
    private ?\PHPUnit\Framework\MockObject\MockObject $dynamic = null;
    private $stub;

    protected function setUp(): void
    {
        $this->a = $this->createMock(\DateTime::class);
        $this->private = $this->createMock(\DateTime::class);
        $this->withDefault = $this->createMock(\DateTime::class);
        $this->wrong = $this->createMock(\DateTime::class);
        $this->union = $this->createMock(\DateTime::class);
        $this->nullable = $this->createMock(\DateTime::class);
        $this->notMock = new \DateTime();
        $this->mixed = $this->createMock(\DateTime::class);
        $this->mixed = new \DateTime();
        $this->stub = $this->createStub(\DateTime::class);
    }

    public function testLater(): void
    {
        $this->later = $this->createMock(\DateTime::class);
        $this->dynamic = $this->createMock(\DateTime::class);
    }
}

final class DynamicTest extends TestCase
{
    private $mock;

    protected function setUp(): void
    {
        $this->mock = $this->createMock(\DateTime::class);
        $name = 'other';
        $this->$name = 1;
    }
}

final class NotATest
{
    private $mock;

    protected function setUp(): void
    {
        $this->mock = $this->createMock(\DateTime::class);
    }
}

final class Mock84Test extends TestCase
{
    private $intersection;
    private $defaultNull = null;
    private ?\PHPUnit\Framework\MockObject\MockObject $resetLater = null;
    private \PHPUnit\Framework\MockObject\MockObject $hooked { get => $this->hooked; }

    protected function setUp(): void
    {
        $this->intersection = $this->make();
        $this->hooked = $this->createMock(\DateTime::class);
    }

    public function testIt(): void
    {
        $this->defaultNull = $this->createMock(\DateTime::class);
        $this->resetLater = $this->createMock(\DateTime::class);
    }

    protected function tearDown(): void
    {
        $this->resetLater = null;
    }

    private function make(): MockObject&\DateTime
    {
        return $this->createMock(\DateTime::class);
    }
}
