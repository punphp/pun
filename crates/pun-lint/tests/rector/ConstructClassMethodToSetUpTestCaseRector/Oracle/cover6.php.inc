<?php

namespace App\Tests\Cover6;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes\Depends;
use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

abstract class AbstractSetUpTest extends TestCase
{
    private MockObject $abstractMock;

    abstract protected function setUp(): void;

    abstract protected function helper();

    #[DataProvider('provide')]
    abstract public function testAbstract($a);
}

final class Cover6Test extends TestCase
{
    private Stub $s;

    private Stub $other;

    private MockObject|Stub|null $three;

    private MockObject&\Countable&\Stringable $tripleIntersection;

    private MockObject $unusedMock;

    protected function setUp(): void
    {
        $this->s = $this->createStub(\stdClass::class);
        $this->three = $this->createStub(\stdClass::class);
        $this->unusedMock = $this->createMock(\stdClass::class);
        $this->unusedMock->method('x');
    }

    public function testStubUses(): void
    {
        $sStub = 1;
        foo($this->s);
        bar($this->s, array($this->three), [...$this->three]);
        [, $a] = [1, 2];
        $name = 'x';
        $this->{$name}->y();
        $other = $this->{$name};
        $x = new \stdClass();
        $y = $x::$name;
    }

    public function testMocks(): void
    {
        $a = $this->createMock(parent::class);
        $b = $this->createMock(static::class);
        $c = $this->createMock(self::class);
        $d = $this->createMock($a::class);
        new \ArrayObject($this->createMock(static::class));
        new \ArrayObject($this->createMock(self::class));
    }

    private function intersected(\stdClass&MockObject $m)
    {
        return $m;
    }

    public function testA()
    {
        return 1;
    }

    #[Depends('')]
    public function testEmptyDepends($a): void
    {
    }

    #[Depends('testMissing')]
    public function testDependsMissing($a): void
    {
    }

    #[Depends('testA')]
    public function testDependsUntyped($a): void
    {
    }

    public function &testByReference(): void
    {
    }

    #[DataProvider('provideNamed')]
    public function testNamed(int $first, int $second): void
    {
    }

    #[DataProvider('provideMissing')]
    public function testNamedMissing(int $first): void
    {
    }

    public static function provideNamed(): array
    {
        $x = 1;
        if ($x) {
        }
        return [
            [1 => 1, 2],
            array(1, 2),
            ...$x,
        ];
    }
}

function outside()
{
    new \ArrayObject($this->createMock(\stdClass::class));
}

final class ConstructorTest extends TestCase
{
    #[\Deprecated]
    public function __construct($name)
    {
        $a = 1;
        // between
        $b = 2;
        parent::__construct($name);
    }
}

final class ReferenceConstructorTest extends TestCase
{
    public function &__construct()
    {
        // only comment
        $a = 1;
        /* block */
        $b = 2;
    }
}
-----
<?php

namespace App\Tests\Cover6;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes\Depends;
use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

abstract class AbstractSetUpTest extends TestCase
{
    private MockObject $abstractMock;

    abstract protected function setUp(): void;

    abstract protected function helper();

    #[DataProvider('provide')]
    abstract public function testAbstract($a);
}

final class Cover6Test extends TestCase
{
    private Stub $s;

    private Stub $other;

    private MockObject|Stub|null $three;

    private MockObject&\Countable&\Stringable $tripleIntersection;

    private MockObject $unusedMock;

    protected function setUp(): void
    {
        $this->s = $this->createStub(\stdClass::class);
        $this->three = $this->createStub(\stdClass::class);
        $this->unusedMock = $this->createMock(\stdClass::class);
        $this->unusedMock->method('x');
    }

    public function testStubUses(): void
    {
        $sStub = 1;
        foo($this->s);
        bar($this->s, array($this->three), [...$this->three]);
        [, $a] = [1, 2];
        $name = 'x';
        $this->{$name}->y();
        $other = $this->{$name};
        $x = new \stdClass();
        $y = $x::$name;
    }

    public function testMocks(): void
    {
        $a = $this->createMock(parent::class);
        $b = $this->createMock(static::class);
        $c = $this->createMock(self::class);
        $d = $this->createMock($a::class);
        new \ArrayObject($this->createMock(static::class));
        new \ArrayObject($this->createMock(self::class));
    }

    private function intersected(\stdClass&MockObject $m)
    {
        return $m;
    }

    public function testA()
    {
        return 1;
    }

    #[Depends('')]
    public function testEmptyDepends($a): void
    {
    }

    #[Depends('testMissing')]
    public function testDependsMissing($a): void
    {
    }

    #[Depends('testA')]
    public function testDependsUntyped($a): void
    {
    }

    public function &testByReference(): void
    {
    }

    #[DataProvider('provideNamed')]
    public function testNamed(int $first, int $second): void
    {
    }

    #[DataProvider('provideMissing')]
    public function testNamedMissing(int $first): void
    {
    }

    public static function provideNamed(): array
    {
        $x = 1;
        if ($x) {
        }
        return [
            [1 => 1, 2],
            array(1, 2),
            ...$x,
        ];
    }
}

function outside()
{
    new \ArrayObject($this->createMock(\stdClass::class));
}

final class ConstructorTest extends TestCase
{
    #[\Deprecated]
    protected function setUp(): void
    {
        $a = 1;
        // between
        $b = 2;
    }
}

final class ReferenceConstructorTest extends TestCase
{
    protected function &setUp(): void
    {
        // only comment
        $a = 1;
        /* block */
        $b = 2;
    }
}
