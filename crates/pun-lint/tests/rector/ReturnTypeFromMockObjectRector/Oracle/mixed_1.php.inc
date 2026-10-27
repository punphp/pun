<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    private MockObject $mock;

    public function a()
    {
        return $this->createMock(\DateTime::class);
    }

    public function b()
    {
        return $this->createStub(\DateTime::class);
    }

    public function c()
    {
        return $this->mock;
    }

    private function d(MockObject&\DateTime $x)
    {
        return $x;
    }

    public function e(MockObject|\DateTime $x)
    {
        return $x;
    }

    public function f()
    {
        $m = $this->getMockBuilder(\DateTime::class)->getMock();
        if (rand(0, 1)) {
            return $m;
        }
        return $m;
    }

    public function g()
    {
        $f = function () { return 1; };
        return $this->createMock(\DateTime::class);
    }

    public static function h(MockObject $m)
    {
        return $m;
    }

    public function setUp(): void {}

    public function i(MockObject&\DateTime&\Countable $x)
    {
        return $x;
    }
}

class NotTest
{
    public function a(MockObject $m)
    {
        return $m;
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    private MockObject $mock;

    public function a(): \PHPUnit\Framework\MockObject\MockObject
    {
        return $this->createMock(\DateTime::class);
    }

    public function b()
    {
        return $this->createStub(\DateTime::class);
    }

    public function c(): \PHPUnit\Framework\MockObject\MockObject
    {
        return $this->mock;
    }

    private function d(MockObject&\DateTime $x): \PHPUnit\Framework\MockObject\MockObject
    {
        return $x;
    }

    public function e(MockObject|\DateTime $x)
    {
        return $x;
    }

    public function f()
    {
        $m = $this->getMockBuilder(\DateTime::class)->getMock();
        if (rand(0, 1)) {
            return $m;
        }
        return $m;
    }

    public function g(): \PHPUnit\Framework\MockObject\MockObject
    {
        $f = function () { return 1; };
        return $this->createMock(\DateTime::class);
    }

    public static function h(MockObject $m): \PHPUnit\Framework\MockObject\MockObject
    {
        return $m;
    }

    public function setUp(): void {}

    public function i(MockObject&\DateTime&\Countable $x)
    {
        return $x;
    }
}

class NotTest
{
    public function a(MockObject $m)
    {
        return $m;
    }
}
