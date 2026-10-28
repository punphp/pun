<?php

namespace Pun\Vm;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class MoreTest extends TestCase
{
    private MockObject $selfMock;
    private MockObject $staticMock;
    private MockObject $parentMock;
    private MockObject $dynamic;
    private MockObject&Manager&Persister $agree;
    private MockObject&Unrelated&Manager $never;

    protected function setUp(): void
    {
        $this->selfMock = $this->createMock(self::class);
        $this->staticMock = $this->createMock(static::class);
        $this->parentMock = $this->createMock(parent::class);
        $this->{'dyn'} = $this->createMock(Manager::class);
        $this->dynamic = $this->createMock($this->name());
        $this->dynamic = $this->createMock(Manager::class);
    }

    public function testMore(object $any): void
    {
        $this->createMock(Helps::class)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Unrelated::class)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $any->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $mocker = $this->createMock(Manager::class)->expects($this->once());
        $mocker->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->selfMock->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->staticMock->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->parentMock->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->{'dyn'}->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->dynamic->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->agree->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->never->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Manager::class)->method('persist')->with($this->callback(function ($e) {
            if ($e) {
                return throw new \LogicException();
            }
            return;
        }));
    }

    public function persist(object $entity): void
    {
    }
}
-----
<?php

namespace Pun\Vm;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class MoreTest extends TestCase
{
    private MockObject $selfMock;
    private MockObject $staticMock;
    private MockObject $parentMock;
    private MockObject $dynamic;
    private MockObject&Manager&Persister $agree;
    private MockObject&Unrelated&Manager $never;

    protected function setUp(): void
    {
        $this->selfMock = $this->createMock(self::class);
        $this->staticMock = $this->createMock(static::class);
        $this->parentMock = $this->createMock(parent::class);
        $this->{'dyn'} = $this->createMock(Manager::class);
        $this->dynamic = $this->createMock($this->name());
        $this->dynamic = $this->createMock(Manager::class);
    }

    public function testMore(object $any): void
    {
        $this->createMock(Helps::class)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Unrelated::class)->method('persist')->willReturnCallback(function ($e): void {
        });
        $any->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $mocker = $this->createMock(Manager::class)->expects($this->once());
        $mocker->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->selfMock->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->staticMock->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->parentMock->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->{'dyn'}->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->dynamic->method('persist')->willReturnCallback(function ($e): void {
        });
        $this->agree->method('persist')->willReturnCallback(function ($e): void {
        });
        $this->never->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Manager::class)->method('persist')->willReturnCallback(function ($e): void {
            if ($e) {
                throw new \LogicException();
            }
        });
    }

    public function persist(object $entity): void
    {
    }
}
