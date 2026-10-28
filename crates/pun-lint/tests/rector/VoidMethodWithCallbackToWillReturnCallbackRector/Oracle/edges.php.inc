<?php

namespace Pun\Vm;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class EdgesTest extends TestCase
{
    private MockObject $built;
    private MockObject $constMock;
    private MockObject $plain;

    protected function setUp(): void
    {
        $this->built = $this->getMockBuilder(Manager::class);
        $this->constMock = $this->createMock(Manager::SOME);
        $shared = $this->createMock(Manager::class);
    }

    public function testEdges(): void
    {
        $m = $this->createMock(Manager::class);
        $m->method('persist')->with($this->callback(function ($e) {
            return (true);
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            echo 1;
            return  true ;
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            if ($e) {
                return /* why */ true;
            }
            echo 1;
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            if ($e) {
                return <<<TXT
                a
                TXT;
            }
            echo 1;
        }));
        $m->method('persist')->with($this->callback(function (?object $e = null, int ...$rest) {
            return true;
        }));
        $m->method('persist')->with($this->callback(static function (object &$e) use (&$m, $e): bool {
            return true;
        }));
        $m->method('persist')->with(
            $this->callback(
                function (
                    $e
                ) {
                    return true;
                }
            ),
        );
        $m->method('persist')->with($this->callback(function ($e) {
            if ($e) return throw new \LogicException();
            echo 1;
            return false;
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            if ($e) {
                return null;
            }
            return throw new \LogicException();
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            $o = new class {
                public function f()
                {
                    return 1;
                }
            };
            return true;
        }));
        $this->built->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->constMock->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $shared = $this->createMock(FinalManager::class);
        $shared->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $this->plain = $this->createMock(Manager::class);
        $this->plain->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
        $other = new \stdClass();
        $other->built->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
    }
}
-----
<?php

namespace Pun\Vm;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class EdgesTest extends TestCase
{
    private MockObject $built;
    private MockObject $constMock;
    private MockObject $plain;

    protected function setUp(): void
    {
        $this->built = $this->getMockBuilder(Manager::class);
        $this->constMock = $this->createMock(Manager::SOME);
        $shared = $this->createMock(Manager::class);
    }

    public function testEdges(): void
    {
        $m = $this->createMock(Manager::class);
        $m->method('persist')->willReturnCallback(function ($e): void {
        });
        $m->method('persist')->willReturnCallback(function ($e): void {
            echo 1;
        });
        $m->method('persist')->willReturnCallback(function ($e): void {
            if ($e) {
                return;
            }
            echo 1;
        });
        $m->method('persist')->willReturnCallback(function ($e): void {
            if ($e) {
                return;
            }
            echo 1;
        });
        $m->method('persist')->willReturnCallback(function (?object $e = null, int ...$rest): void {
        });
        $m->method('persist')->willReturnCallback(static function (object &$e) use (&$m, $e): void {
        });
        $m->method('persist')->willReturnCallback(
            function ($e): void {
            },
        );
        $m->method('persist')->willReturnCallback(function ($e): void {
            if ($e) throw new \LogicException();
            echo 1;
        });
        $m->method('persist')->willReturnCallback(function ($e): void {
            if ($e) {
                return;
            }
            return throw new \LogicException();
        });
        $m->method('persist')->willReturnCallback(function ($e): void {
            $o = new class {
                public function f()
                {
                    return;
                }
            };
        });
        $this->built->method('persist')->willReturnCallback(function ($e): void {
        });
        $this->constMock->method('persist')->willReturnCallback(function ($e): void {
        });
        $shared = $this->createMock(FinalManager::class);
        $shared->method('persist')->willReturnCallback(function ($e): void {
        });
        $this->plain = $this->createMock(Manager::class);
        $this->plain->method('persist')->willReturnCallback(function ($e): void {
        });
        $other = new \stdClass();
        $other->built->method('persist')->with($this->callback(function ($e) {
            return true;
        }));
    }
}
