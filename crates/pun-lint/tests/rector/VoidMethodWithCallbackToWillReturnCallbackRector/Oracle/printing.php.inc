<?php

namespace Pun\Vm;

use PHPUnit\Framework\TestCase;

const FLAG = true;

final class PrintingTest extends TestCase
{
    public function testShapes($x): void
    {
        $m = $this->createMock(Manager::class);
        $m->method('persist')->with($this->callback(function($e)use($x){return true;}));
        $m->method('persist')->with($this->callback(static function ($e) use ($x): bool {
            if ($e) {
                return false;
            }
            // trailing note
            return true;
        }));
        $m->method('persist')->with(
            $this->callback(
                function ($e) {
                    echo $e;

                    return true;
                }
            )
        );
        $m->method('persist')->with(constraint: $this->callback(function ($e) {
            echo $e;
            return true;
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            echo $e;
            return throw new \RuntimeException('boom');
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            if ($e) {
                // why
                return throw new \RuntimeException('boom');
            }
            echo $e;
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            if ($e) {
                return;
            }
            return "a$e";
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            if ($e) {
                return FLAG;
            }
            if (!$e) {
                return __LINE__;
            }
            return $e;
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            return -1;
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            return $this->x();
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            echo $e;
            return;
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            $f = fn () => 1;
            return true;
        }));
        $m->method('persist')->with($this->callback(fn ($e) => true));
        $m->method('persist')->with($this->callback());
        $m->method('persist')->with($this->callback(...));
        $m->method('persist')->with($this->callback(function ($e) {
            return true;
        }), 1);
        $m->method('persist')->WITH($this->CALLBACK(function ($e) {
            return null;
        }))->willReturn(null);
        $m->method('persist')->with($this->callback(function ($e) {
            return true; // done
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            echo 1;
            /** doc */
            return true;
        }));
        $m->method()->with($this->callback(function ($e) {
            return true;
        }));
        $m->method("persist")->with($this->callback(function ($e) {
            return true;
        }));
        $m->method($name)->with($this->callback(function ($e) {
            return true;
        }));
    }
}
-----
<?php

namespace Pun\Vm;

use PHPUnit\Framework\TestCase;

const FLAG = true;

final class PrintingTest extends TestCase
{
    public function testShapes($x): void
    {
        $m = $this->createMock(Manager::class);
        $m->method('persist')->willReturnCallback(function ($e) use ($x): void {
        });
        $m->method('persist')->willReturnCallback(static function ($e) use ($x): void {
            if ($e) {
                return;
            }
        });
        $m->method('persist')->willReturnCallback(
            function ($e): void {
                echo $e;
            }
        );
        $m->method('persist')->willReturnCallback(function ($e): void {
            echo $e;
        });
        $m->method('persist')->willReturnCallback(function ($e): void {
            echo $e;
            return throw new \RuntimeException('boom');
        });
        $m->method('persist')->willReturnCallback(function ($e): void {
            if ($e) {
                throw new \RuntimeException('boom');
            }
            echo $e;
        });
        $m->method('persist')->willReturnCallback(function ($e): void {
            if ($e) {
                return;
            }
        });
        $m->method('persist')->willReturnCallback(function ($e): void {
            if ($e) {
                return;
            }
            if (!$e) {
                return;
            }
        });
        $m->method('persist')->with($this->callback(function ($e) {
            return -1;
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            return $this->x();
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            echo $e;
            return;
        }));
        $m->method('persist')->with($this->callback(function ($e) {
            $f = fn () => 1;
            return true;
        }));
        $m->method('persist')->with($this->callback(fn ($e) => true));
        $m->method('persist')->with($this->callback());
        $m->method('persist')->with($this->callback(...));
        $m->method('persist')->with($this->callback(function ($e) {
            return true;
        }), 1);
        $m->method('persist')->willReturnCallback(function ($e): void {
        })->willReturn(null);
        $m->method('persist')->willReturnCallback(function ($e): void {
            return; // done
        });
        $m->method('persist')->willReturnCallback(function ($e): void {
            echo 1;
        });
        $m->method()->with($this->callback(function ($e) {
            return true;
        }));
        $m->method("persist")->willReturnCallback(function ($e): void {
        });
        $m->method($name)->with($this->callback(function ($e) {
            return true;
        }));
    }
}
