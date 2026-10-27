<?php

namespace App\Tests\Cover;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes\Test;
use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

interface SomeInterface
{
    public function testIt(): void;
}

enum SomeEnum
{
    case A;

    public function testIt(): void
    {
    }
}

trait SomeTrait
{
    public function helperInTrait(): void
    {
        $this->assertSame(1, $this->value);
    }
}

final class NotATest
{
    private $value;

    public function testIt($a, $b): void
    {
        $this->assertSame(1, count($a));
        $this->assertEquals(true, $a);
        $this->assertTrue(isset($a['x']));
        $this->assertTrue($a instanceof \stdClass);
        $this->assertFalse(strpos($a, 'x'));
        $this->assertTrue($a === $b);
        $mock = $this->createMock(\stdClass::class);
        $mock->expects($this->any())->method('x');
        $this->assertSame([], (array) $a);
    }
}

abstract class AbstractCoverTest extends TestCase
{
    const SOME = 1;

    private $value;

    private int $typed = 0;

    abstract public function testAbstract(): void;

    abstract protected function provider(): array;
}

final class CoverTest extends TestCase
{
    use SomeTrait;

    const SOME = 1;

    private $value;

    public function testCalls($a, $b, $object, $method): void
    {
        $this->assertSame();
        $this->assertSame(...);
        $this->assertEquals(...$a);
        $this->assertTrue(...$a);
        $this->{$method}(1, 2);
        $this->$method(1);
        $object->assertSame(1, $a);
        $object::assertSame(1, $a);
        parent::assertSame(1, $a);
        static::assertSame(1, $a);
        self::assertSame(expected: 1, actual: $a);
        $this->assertSame(actual: $a, expected: 1);
        $this->assertTrue(!$a, 'message', 'extra');
        $this->assertFalse(!$a);
        $this->assertTrue(isset($a));
        $this->assertTrue(isset($a->b));
        $this->assertFalse(isset($a['x'], $a['y']));
        $this->assertTrue($a instanceof $b);
        $this->assertFalse($a instanceof \stdClass);
        $this->assertTrue(is_a($a, 'stdClass'));
        $this->assertEquals(count($b), 2);
        $this->assertSame(2, $b->count());
        $this->assertSame(2, sizeof($a));
        $this->assertEquals('2', $a);
        $this->assertEquals(2.5, $a);
        $this->assertEquals(2.5, $a, 'message', 0.1);
        $this->assertEquals([], $a);
        $this->assertNotEquals(null, $a);
        $this->assertSame(null, $a);
        $this->assertSame(false, $a);
        $this->assertNotSame(true, $a);
        $this->assertTrue($a == $b);
        $this->assertTrue($a !== $b);
        $this->assertFalse($a < $b);
        $this->assertTrue($a <=> $b);
        $this->assertTrue(in_array($a, $b));
        $this->assertTrue(is_string($a));
        $this->assertFalse(empty($a));
        $this->assertTrue(strpos($a, 'x') !== false);
        $this->assertFalse(stripos($a, 'x'));
        $this->assertContains(1, $a);
        $this->assertStringContainsString($a, 1);
        $this->assertContainsOnly('string', $a);
        $this->assertIsType('string', $a);
        assert($a === 1);
        \assert($a);
        $fn = fn () => $this->assertTrue($a);
        $closure = function () use ($a) {
            $this->assertTrue($a);
        };
        new class {
            public function testInner(): void
            {
            }
        };
    }

    public function testMocks($a): void
    {
        $mock = $this->createMock(\stdClass::class);
        $mock->expects($this->any())->method('x');
        $mock->expects($this->at(0))->method('x');
        $mock->expects($a)->method('x');
        $mock->method('x')->with($this->equalTo(1));
        $mock->method('x')->with($this->callback(fn ($x) => true));
        $mock->method('x')->willReturnCallback(fn () => 1);
        $mock->method('x')->will($this->returnValue(1));
        $mock->method('x')->will($a);
        $mock->method('x')->withConsecutive([1]);
        $builder = $this->getMockBuilder(\stdClass::class)->getMock();
        $this->createMock(\stdClass::class);
        $other = $this->createMock($a);
        $this->value = $this->createMock(\stdClass::class);
        $this->value->method('x');
    }

    /**
     * @dataProvider provideData
     * @depends testCalls
     * @covers \stdClass
     * @test
     */
    public function documented(int $a): void
    {
    }

    #[DataProvider('provideData')]
    #[Test]
    public function attributed($a)
    {
        return $a;
    }

    public static function provideData(): iterable
    {
        yield [1];
        yield 'named' => [2];
    }

    protected function setUp(): void
    {
        parent::setUp();
    }

    private function helper(MockObject $mock, $other)
    {
        $mock->expects($this->once());
        return $other;
    }
}
-----
<?php

namespace App\Tests\Cover;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes\Test;
use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

interface SomeInterface
{
    public function testIt(): void;
}

enum SomeEnum
{
    case A;

    public function testIt(): void
    {
    }
}

trait SomeTrait
{
    public function helperInTrait(): void
    {
        $this->assertSame(1, $this->value);
    }
}

final class NotATest
{
    private $value;

    public function testIt($a, $b): void
    {
        $this->assertSame(1, count($a));
        $this->assertEquals(true, $a);
        $this->assertTrue(isset($a['x']));
        $this->assertTrue($a instanceof \stdClass);
        $this->assertFalse(strpos($a, 'x'));
        $this->assertTrue($a === $b);
        $mock = $this->createMock(\stdClass::class);
        $mock->expects($this->any())->method('x');
        $this->assertSame([], (array) $a);
    }
}

abstract class AbstractCoverTest extends TestCase
{
    const SOME = 1;

    private $value;

    private int $typed = 0;

    abstract public function testAbstract(): void;

    abstract protected function provider(): array;
}

final class CoverTest extends TestCase
{
    use SomeTrait;

    const SOME = 1;

    private $value;

    public function testCalls($a, $b, $object, $method): void
    {
        $this->assertSame();
        $this->assertSame(...);
        $this->assertEquals(...$a);
        $this->assertTrue(...$a);
        $this->{$method}(1, 2);
        $this->$method(1);
        $object->assertSame(1, $a);
        $object::assertSame(1, $a);
        parent::assertSame(1, $a);
        static::assertSame(1, $a);
        self::assertSame(expected: 1, actual: $a);
        $this->assertSame(actual: $a, expected: 1);
        $this->assertTrue(!$a, 'message', 'extra');
        $this->assertFalse(!$a);
        $this->assertTrue(isset($a));
        $this->assertTrue(isset($a->b));
        $this->assertFalse(isset($a['x'], $a['y']));
        $this->assertTrue($a instanceof $b);
        $this->assertFalse($a instanceof \stdClass);
        $this->assertTrue(is_a($a, 'stdClass'));
        $this->assertEquals(count($b), 2);
        $this->assertSame(2, $b->count());
        $this->assertSame(2, sizeof($a));
        $this->assertEquals('2', $a);
        $this->assertEquals(2.5, $a);
        $this->assertEquals(2.5, $a, 'message', 0.1);
        $this->assertEquals([], $a);
        $this->assertNotEquals(null, $a);
        $this->assertSame(null, $a);
        $this->assertSame(false, $a);
        $this->assertNotSame(true, $a);
        $this->assertTrue($a == $b);
        $this->assertTrue($a !== $b);
        $this->assertFalse($a < $b);
        $this->assertTrue($a <=> $b);
        $this->assertTrue(in_array($a, $b));
        $this->assertTrue(is_string($a));
        $this->assertFalse(empty($a));
        $this->assertTrue(strpos($a, 'x') !== false);
        $this->assertFalse(stripos($a, 'x'));
        $this->assertContains(1, $a);
        $this->assertStringContainsString($a, 1);
        $this->assertContainsOnly('string', $a);
        $this->assertIsType('string', $a);
        assert($a === 1);
        \assert($a);
        $fn = fn () => $this->assertTrue($a);
        $closure = function () use ($a) {
            $this->assertTrue($a);
        };
        new class {
            public function testInner(): void
            {
            }
        };
    }

    public function testMocks($a): void
    {
        $mock = $this->createMock(\stdClass::class);
        $mock->expects($this->any())->method('x');
        $mock->expects($this->at(0))->method('x');
        $mock->expects($a)->method('x');
        $mock->method('x')->with($this->equalTo(1));
        $mock->method('x')->with($this->callback(fn ($x) => true));
        $mock->method('x')->willReturnCallback(fn () => 1);
        $mock->method('x')->will($this->returnValue(1));
        $mock->method('x')->will($a);
        $mock->method('x')->withConsecutive([1]);
        $builder = $this->getMockBuilder(\stdClass::class)->getMock();
        $this->createMock(\stdClass::class);
        $other = $this->createMock($a);
        $this->value = $this->createMock(\stdClass::class);
        $this->value->method('x');
    }

    /**
     * @dataProvider provideData
     * @depends testCalls
     * @covers \stdClass
     * @test
     */
    public function documented(int $a): void
    {
    }

    #[DataProvider('provideData')]
    #[Test]
    public function attributed($a)
    {
        return $a;
    }

    public static function provideData(): iterable
    {
        yield [1];
        yield 'named' => [2];
    }

    protected function setUp(): void
    {
        parent::setUp();
    }

    private function helper(MockObject $mock, $other)
    {
        $mock->expects($this->once());
        return $other;
    }
}
