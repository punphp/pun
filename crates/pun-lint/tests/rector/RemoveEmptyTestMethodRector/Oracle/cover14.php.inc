<?php

namespace App\Tests\Cover14;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

function outside()
{
    new \ArrayObject($this->createMock(\Symfony\Component\HttpFoundation\Request::class));
}

final class BodilessSetUpTest extends TestCase
{
    private MockObject $m;

    abstract protected function setUp(): void;
}

final class Cover14Test extends TestCase
{
    /**
     * Text.

     * @dataProvider provide
     */
    public function testBlankLine($a): void
    {
    }

    /**
     * Text.
     * @depends testA
     */
    public function testCarriageReturn($a): void
    {
    }

    /**
       Text.
       @dataProvider provide

     */
    public function testUnstarred($a): void
    {
    }

    public static function provide(): array
    {
        return [[1]];
    }

    public function testA(): int
    {
        return 1;
    }
}
-----
<?php

namespace App\Tests\Cover14;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

function outside()
{
    new \ArrayObject($this->createMock(\Symfony\Component\HttpFoundation\Request::class));
}

final class BodilessSetUpTest extends TestCase
{
    private MockObject $m;

    abstract protected function setUp(): void;
}

final class Cover14Test extends TestCase
{
    public static function provide(): array
    {
        return [[1]];
    }

    public function testA(): int
    {
        return 1;
    }
}
