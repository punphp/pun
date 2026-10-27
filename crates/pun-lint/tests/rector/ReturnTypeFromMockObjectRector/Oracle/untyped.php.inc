<?php

namespace App\Tests\ReturnMock;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class ReturnMockTest extends TestCase
{
    public function testUntypedParameter($value)
    {
        return $value;
    }

    private function intersection(): MockObject&\stdClass
    {
        return $this->createMock(\stdClass::class);
    }

    private function returnsIntersection()
    {
        return $this->intersection();
    }

    private function untypedParameter($value)
    {
        return $value;
    }
}

abstract class AbstractReturnMockTest extends TestCase
{
    abstract public function testAbstract();
}
-----
<?php

namespace App\Tests\ReturnMock;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class ReturnMockTest extends TestCase
{
    public function testUntypedParameter($value)
    {
        return $value;
    }

    private function intersection(): MockObject&\stdClass
    {
        return $this->createMock(\stdClass::class);
    }

    private function returnsIntersection(): \PHPUnit\Framework\MockObject\MockObject
    {
        return $this->intersection();
    }

    private function untypedParameter($value)
    {
        return $value;
    }
}

abstract class AbstractReturnMockTest extends TestCase
{
    abstract public function testAbstract();
}
