<?php

namespace App\Tests\Cover7;

use App\Elsewhere\Thing as Aliased;
use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes\Group;
use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

class Marker
{
}

final class Cover7Test extends TestCase
{
    /**
     * @var MockObject
     */
    public Stub $hooked {
        get => $this->hooked;
    }

    public Stub $undocumented;

    /**
     * @see 1_000 1.5_0 1_0.5 0b1 0o7 0x1F * 0B1 0O7 0X1F
     * @test
     */
    public function numbers(): void
    {
    }

    /**
     * @Marker
     * @Aliased
     * @dataProvider provide
     */
    public function testAnnotationClasses($a): void
    {
    }

    /**
     * @dataProvider provide
     */
    #[Group(...['x'])]
    public function testSpreadGroup($a): void
    {
    }

    public function testCalls(): void
    {
        $m = 'assertTrue';
        static::{$m}(true);
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->with($this->callback(fn ($x) => $x instanceof SELF));
        $mock->method('x')->with($this->callback(fn ($x) => $x instanceof Static));
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

namespace App\Tests\Cover7;

use App\Elsewhere\Thing as Aliased;
use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes\Group;
use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

class Marker
{
}

final class Cover7Test extends TestCase
{
    /**
     * @var MockObject
     */
    public Stub $hooked {
        get => $this->hooked;
    }

    public Stub $undocumented;

    /**
     * @see 1_000 1.5_0 1_0.5 0b1 0o7 0x1F * 0B1 0O7 0X1F
     * @test
     */
    public function numbers(): void
    {
    }

    /**
     * @Marker
     * @Aliased
     * @dataProvider provide
     */
    public function testAnnotationClasses($a): void
    {
    }

    /**
     * @dataProvider provide
     */
    #[Group(...['x'])]
    public function testSpreadGroup($a): void
    {
    }

    public function testCalls(): void
    {
        $m = 'assertTrue';
        static::{$m}(true);
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->with($this->callback(function ($x): void {
            $this->assertInstanceOf(SELF::class, $x);
            return;
        }));
        $mock->method('x')->with($this->callback(function ($x): void {
            $this->assertInstanceOf(Static::class, $x);
            return;
        }));
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
