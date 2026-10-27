<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    public function testOne(): void
    {
        $a = $this->createMock(\stdClass::class);
        $b = self::createMock(Foo::class);
        $c = $this->createMock(\stdClass::class);
        $c = $this->createMock(\stdClass::class);
        $this->prepare($a, $b, $this->createMock(Bar::class), $c, $a);
        $this->prepare($a, $b, $this->createMock(Bar::class), $c, $a);
        $this->documented($a, $a);
        $this->byRef($a);
        $this->named(mock: $a);
        $this->spread(...$a);
        $fn = function () use ($a) {
            $this->other($a);
        };
        $this->other($a);
    }

    /**
     * Prepare.
     */
    private function prepare(MockObject $a, MockObject $b, MockObject $c, MockObject $d, $e): void
    {
    }

    /**
     * @param MockObject $a
     */
    private function documented(MockObject $a, MockObject $b): void
    {
    }

    private function byRef(MockObject &$a): void
    {
    }

    private function named(MockObject $mock): void
    {
    }

    private function spread(MockObject ...$mock): void
    {
    }

    private function other(MockObject $mock): void
    {
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    public function testOne(): void
    {
        $a = $this->createMock(\stdClass::class);
        $b = self::createMock(Foo::class);
        $c = $this->createMock(\stdClass::class);
        $c = $this->createMock(\stdClass::class);
        $this->prepare($a, $b, $this->createMock(Bar::class), $c, $a);
        $this->prepare($a, $b, $this->createMock(Bar::class), $c, $a);
        $this->documented($a, $a);
        $this->byRef($a);
        $this->named(mock: $a);
        $this->spread(...$a);
        $fn = function () use ($a) {
            $this->other($a);
        };
        $this->other($a);
    }

    /**
     * Prepare.
     * @param \stdClass&\PHPUnit\Framework\MockObject\MockObject $a
     * @param \App\Tests\Foo&\PHPUnit\Framework\MockObject\MockObject $b
     * @param \App\Tests\Bar&\PHPUnit\Framework\MockObject\MockObject $c
     */
    private function prepare(MockObject $a, MockObject $b, MockObject $c, MockObject $d, $e): void
    {
    }

    /**
     * @param MockObject $a
     * @param \stdClass&\PHPUnit\Framework\MockObject\MockObject $b
     */
    private function documented(MockObject $a, MockObject $b): void
    {
    }

    /**
     * @param \stdClass&\PHPUnit\Framework\MockObject\MockObject $a
     */
    private function byRef(MockObject &$a): void
    {
    }

    private function named(MockObject $mock): void
    {
    }

    private function spread(MockObject ...$mock): void
    {
    }

    /**
     * @param \stdClass&\PHPUnit\Framework\MockObject\MockObject $mock
     */
    private function other(MockObject $mock): void
    {
    }
}
