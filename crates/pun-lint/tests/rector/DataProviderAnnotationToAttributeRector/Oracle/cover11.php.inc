<?php

namespace App\Tests\Cover11;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

function outside()
{
    foo($this->createMock(\Symfony\Component\HttpFoundation\Request::class));
}

final class Cover11Test extends TestCase
{
    /**
     * @var MockObject,Foo
     */
    private Stub $commaVar;

    public function testCalls($a): void
    {
        $this->assertSame(
            $a,
            'x'
        );
        $f = ['a' => 1];
        $e = $f;
        $this->assertEquals($e, ['a' => 1]);
        $this->assertStringContainsString('a', $a + 1);
        $const = $this->createMock(\stdClass::FOO);
        new \ArrayObject($const);
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->with($this->callback(function ($x) use ($a) {
            $y = 1;
            return $x === $a;
        }));
        $mock->method('x')->with($this->callback(function ($x) {
            return $x instanceof SELF;
        }));
    }

    /**
     * @1
     * @dataProvider provide
     */
    #[DataProvider('provide')]
    public function testFirst(int $first): void
    {
    }

    public static function provide(): array
    {
        return [[1]];
        return $x;
    }
}

final class InlineDynTest extends TestCase
{
    private Stub $s;

    protected function setUp(): void
    {
        $this->s = $this->createStub(\stdClass::class);
    }

    public function testX(): void
    {
        $name = 's';
        foo($this->s);
        foo($this->{$name});
    }
}

final class AbstractSetUpInFinalTest extends TestCase
{
    private MockObject $m;

    abstract protected function setUp(): void;
}

final class RefConstructorTest extends TestCase
{
    public function &__construct($name)
    {
        $a = 1;
    }
}

final class ConstructorAndSetUpTest extends TestCase
{
    public function __construct()
    {
        // first
        $a = 1;
        /* second */
        $b = 2;
    }

    protected function setUp(): void
    {
    }
}
-----
<?php

namespace App\Tests\Cover11;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

function outside()
{
    foo($this->createMock(\Symfony\Component\HttpFoundation\Request::class));
}

final class Cover11Test extends TestCase
{
    /**
     * @var MockObject,Foo
     */
    private Stub $commaVar;

    public function testCalls($a): void
    {
        $this->assertSame(
            $a,
            'x'
        );
        $f = ['a' => 1];
        $e = $f;
        $this->assertEquals($e, ['a' => 1]);
        $this->assertStringContainsString('a', $a + 1);
        $const = $this->createMock(\stdClass::FOO);
        new \ArrayObject($const);
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->with($this->callback(function ($x) use ($a) {
            $y = 1;
            return $x === $a;
        }));
        $mock->method('x')->with($this->callback(function ($x) {
            return $x instanceof SELF;
        }));
    }

    /**
     * @1
     */
    #[DataProvider('provide')]
    public function testFirst(int $first): void
    {
    }

    public static function provide(): array
    {
        return [[1]];
        return $x;
    }
}

final class InlineDynTest extends TestCase
{
    private Stub $s;

    protected function setUp(): void
    {
        $this->s = $this->createStub(\stdClass::class);
    }

    public function testX(): void
    {
        $name = 's';
        foo($this->s);
        foo($this->{$name});
    }
}

final class AbstractSetUpInFinalTest extends TestCase
{
    private MockObject $m;

    abstract protected function setUp(): void;
}

final class RefConstructorTest extends TestCase
{
    public function &__construct($name)
    {
        $a = 1;
    }
}

final class ConstructorAndSetUpTest extends TestCase
{
    public function __construct()
    {
        // first
        $a = 1;
        /* second */
        $b = 2;
    }

    protected function setUp(): void
    {
    }
}
