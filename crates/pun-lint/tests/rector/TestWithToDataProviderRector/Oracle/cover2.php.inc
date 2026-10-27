<?php

namespace App\Tests\Cover2 {

use PHPUnit\Framework\{TestCase, Attributes\Group};
use PHPUnit\Framework\MockObject\{MockObject, Stub};
use function PHPUnit\Framework\{assertTrue, assertSame};
use PHPUnit\Framework\Attributes\{Test, function someFunction};
use function strlen;
use PHPUnit\Framework\Attributes as A;

abstract class AbstractCover2Test extends TestCase
{
    private MockObject $first, $second;

    private ?MockObject $nullable = null;

    private MockObject|Stub $union;

    private MockObject&\stdClass $intersection;

    public \stdClass $hooked {
        get => $this->hooked;
    }

    abstract protected function setUp(): void;

    abstract protected function tearDown(): void;

    /**
     * @dataProvider provideAbstract
     * @depends testOther
     */
    abstract public function testAbstractWithDoc(int $a): void;

    abstract public static function provideAbstract(): iterable;

    /**
     * @test
     * @covers \stdClass::method
     * @ticket 123
     * @requires PHP 8.0
     * @testWith [1, 2]
     * @group cover
     */
    public function commented(int $a, int $b): void
    {
    }

    /**
     * @covers ::method
     * @covers \stdClass
     * @depends testOther
     * @depends
     * @dataProvider
     * @testWith ["a\nb", "\t", "\u0008", "\f", "\u0001"]
     *           [-1.5e30, 0.0, -0.0]
     * @requires extension mysqli
     * @ticket
     * @uses \stdClass
     * @A\Test
     */
    public function testDocs($a, $b)
    {
        // a comment
        $this->first->expects($this->once())->method('x')->with($this->callback(function ($x) use ($a): bool {
            return true;
        }))->willReturnCallback(function ($x) use ($a): int {
            return 1;
        });
        $this->second->method('x')->with($this->callback(function ($x) {
            $this->assertSame(1, $x);
            $this->assertTrue($x);
            return true;
        }));
        $this->nullable?->method('x');
        static::$static = $this->createMock(\stdClass::class);
        $list = [$this->createMock(\stdClass::class), ...$a];
        $this->assertSame($a, strlen($b));
        $this->assertInstanceOf(\stdClass::class, $a, 'message');
        foreach ($a as $item):
            $this->assertInstanceOf(\stdClass::class, $item);
        endforeach;
        foreach ($a as $item) {
            $this->assertInstanceOf($b, $item);
            $this->assertTrue($item);
        }
        return $this->createMock(\stdClass::class);
    }

    /**
     * @testWith [1]
     *           [2]
     */
    public function testTabbed($a): void
    {
        $this->assertTrue($a);
    }

    public function &testReference(): array
    {
        static $x = [];
        return $x;
    }

    #[A\Test]
    #[\PHPUnit\Framework\Attributes\DataProvider(methodName: 'provideAbstract')]
    #[Group('x')]
    public function testAttributes(int $a): void
    {
        $this->assertSame(1, $a);
    }
}

}
-----
<?php

namespace App\Tests\Cover2 {

use PHPUnit\Framework\{TestCase, Attributes\Group};
use PHPUnit\Framework\MockObject\{MockObject, Stub};
use function PHPUnit\Framework\{assertTrue, assertSame};
use PHPUnit\Framework\Attributes\{Test, function someFunction};
use function strlen;
use PHPUnit\Framework\Attributes as A;

abstract class AbstractCover2Test extends TestCase
{
    private MockObject $first, $second;

    private ?MockObject $nullable = null;

    private MockObject|Stub $union;

    private MockObject&\stdClass $intersection;

    public \stdClass $hooked {
        get => $this->hooked;
    }
    public function dataProviderTabbed()
    {
        return [[1], [2]];
    }
    public function dataProviderDocs()
    {
        return [["a\nb", "\t", "\b", "\f", "\u0001"], [-1.5e+30, 0.0, -0.0]];
    }
    public function dataProviderCommented()
    {
        return [[1, 2]];
    }

    abstract protected function setUp(): void;

    abstract protected function tearDown(): void;

    /**
     * @dataProvider provideAbstract
     * @depends testOther
     */
    abstract public function testAbstractWithDoc(int $a): void;

    abstract public static function provideAbstract(): iterable;

    /**
     * @test
     * @covers \stdClass::method
     * @ticket 123
     * @requires PHP 8.0
     * @group cover
     * @dataProvider dataProviderCommented
     */
    public function commented(int $a, int $b): void
    {
    }

    /**
     * @covers ::method
     * @covers \stdClass
     * @depends testOther
     * @depends
     * @dataProvider
     * @requires extension mysqli
     * @ticket
     * @uses \stdClass
     * @A\Test
     * @dataProvider dataProviderDocs
     */
    public function testDocs($a, $b)
    {
        // a comment
        $this->first->expects($this->once())->method('x')->with($this->callback(function ($x) use ($a): bool {
            return true;
        }))->willReturnCallback(function ($x) use ($a): int {
            return 1;
        });
        $this->second->method('x')->with($this->callback(function ($x) {
            $this->assertSame(1, $x);
            $this->assertTrue($x);
            return true;
        }));
        $this->nullable?->method('x');
        static::$static = $this->createMock(\stdClass::class);
        $list = [$this->createMock(\stdClass::class), ...$a];
        $this->assertSame($a, strlen($b));
        $this->assertInstanceOf(\stdClass::class, $a, 'message');
        foreach ($a as $item):
            $this->assertInstanceOf(\stdClass::class, $item);
        endforeach;
        foreach ($a as $item) {
            $this->assertInstanceOf($b, $item);
            $this->assertTrue($item);
        }
        return $this->createMock(\stdClass::class);
    }

    /**
     * @dataProvider dataProviderTabbed
     */
    public function testTabbed($a): void
    {
        $this->assertTrue($a);
    }

    public function &testReference(): array
    {
        static $x = [];
        return $x;
    }

    #[A\Test]
    #[\PHPUnit\Framework\Attributes\DataProvider(methodName: 'provideAbstract')]
    #[Group('x')]
    public function testAttributes(int $a): void
    {
        $this->assertSame(1, $a);
    }
}

}
