<?php

namespace Pun\Vm;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class SetUpTwiceTest extends TestCase
{
    private MockObject $twice;

    protected function setUp(): void
    {
        parent::setUp();
        $this->twice = $this->createMock(Unknown::class);
        $this->twice = $this->createMock(Manager::class);
    }

    public function testUse(): void
    {
        $x = 1;
        $this->twice->method('persist')->with($this->callback(function ($e) use ($x) {
            $x++;
            return true;
        }));
    }
}
-----
<?php

namespace Pun\Vm;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class SetUpTwiceTest extends TestCase
{
    private MockObject $twice;

    protected function setUp(): void
    {
        parent::setUp();
        $this->twice = $this->createMock(Unknown::class);
        $this->twice = $this->createMock(Manager::class);
    }

    public function testUse(): void
    {
        $x = 1;
        $this->twice->method('persist')->willReturnCallback(function ($e) use ($x): void {
            $x++;
        });
    }
}
