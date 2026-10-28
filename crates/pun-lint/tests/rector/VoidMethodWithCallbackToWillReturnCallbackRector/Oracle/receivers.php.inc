<?php

namespace Pun\Vm;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class ReceiversTest extends TestCase
{
    private MockObject $setUpMock;
    private MockObject $setUpFinal;
    private MockObject $plain;
    private MockObject&Manager $typed;
    private $untyped;

    protected function setUp(): void
    {
        $this->setUpMock = $this->createMock(Manager::class);
        $this->setUpFinal = $this->createMock(FinalManager::class);
        $local = $this->getMockBuilder(Persister::class);
        $this->untyped = $this->createMock(Manager::class);
    }

    public function testInline(string $cls): void
    {
        $this->createMock(Manager::class)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Persister::class)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(FinalManager::class)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Kind::class)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock($cls)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Unknown::class)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createStub(Manager::class)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Child::class)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->getMockBuilder(Manager::class)->getMock()->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Manager::class)->method('count')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Manager::class)->method('maybe')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Manager::class)->method('PERSIST')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Manager::class)->method('undefined')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Manager::class)->expects($this->once())->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
    }

    public function testProperties(): void
    {
        $this->setUpMock->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->setUpMock->expects($this->once())->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->setUpFinal->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->plain->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->typed->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->untyped->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $local = $this->createMock(Manager::class);
        $local->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $final = $this->createMock(FinalManager::class);
        $final->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
    }
}
-----
<?php

namespace Pun\Vm;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class ReceiversTest extends TestCase
{
    private MockObject $setUpMock;
    private MockObject $setUpFinal;
    private MockObject $plain;
    private MockObject&Manager $typed;
    private $untyped;

    protected function setUp(): void
    {
        $this->setUpMock = $this->createMock(Manager::class);
        $this->setUpFinal = $this->createMock(FinalManager::class);
        $local = $this->getMockBuilder(Persister::class);
        $this->untyped = $this->createMock(Manager::class);
    }

    public function testInline(string $cls): void
    {
        $this->createMock(Manager::class)->method('persist')->willReturnCallback(function ($e): void {
        });
        $this->createMock(Persister::class)->method('persist')->willReturnCallback(function ($e): void {
        });
        $this->createMock(FinalManager::class)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Kind::class)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock($cls)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Unknown::class)->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createStub(Manager::class)->method('persist')->willReturnCallback(function ($e): void {
        });
        $this->createMock(Child::class)->method('persist')->willReturnCallback(function ($e): void {
        });
        $this->getMockBuilder(Manager::class)->getMock()->method('persist')->willReturnCallback(function ($e): void {
        });
        $this->createMock(Manager::class)->method('count')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Manager::class)->method('maybe')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Manager::class)->method('PERSIST')->willReturnCallback(function ($e): void {
        });
        $this->createMock(Manager::class)->method('undefined')->with($this->callback(function ($e) {
            return true;
        }));
        $this->createMock(Manager::class)->expects($this->once())->method('persist')->willReturnCallback(function ($e): void {
        });
    }

    public function testProperties(): void
    {
        $this->setUpMock->method('persist')->willReturnCallback(function ($e): void {
        });
        $this->setUpMock->expects($this->once())->method('persist')->willReturnCallback(function ($e): void {
        });
        $this->setUpFinal->method('persist')->willReturnCallback(function ($e): void {
        });
        $this->plain->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->typed->method('persist')->willReturnCallback(function ($e): void {
        });
        $this->untyped->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $local = $this->createMock(Manager::class);
        $local->method('persist')->willReturnCallback(function ($e): void {
        });
        $final = $this->createMock(FinalManager::class);
        $final->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
    }
}
