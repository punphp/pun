<?php

namespace App\Tests\SameScalars;

use PHPUnit\Framework\TestCase;

final class SameScalarsTest extends TestCase
{
    private const LIST = ['a' => 1, 'b' => 2];

    public function testArrays(): void
    {
        $this->assertEquals(['a' => 1, 'b' => 2], ['b' => 2, 'a' => 1]);
        $this->assertEquals(['a' => 1, 'b' => 2], ['a' => 1, 'b' => 2]);
        $this->assertEquals(array(1, 2), array(1, 2));
        $this->assertEquals([5 => 'x', 'y'], [5 => 'x', 6 => 'y']);
        $this->assertEquals([1, 2], [1, 3 => 2]);
        $this->assertEquals(self::LIST, ['a' => 1, 'b' => 2]);
        $this->assertEquals(['a' => 1, 'b' => 2], self::LIST);
        $expected = ['x' => 1, 'y' => 2];
        $actual = ['y' => 2, 'x' => 1];
        $this->assertEquals($expected, $actual);
        $same = ['x' => 1, 'y' => 2];
        $this->assertEquals($expected, $same);
        $this->assertEquals([1, 2], [1, ...[2]]);
        $this->assertEquals([1.5 => 'a'], [1 => 'a']);
        $this->assertEquals([true => 'a'], [1 => 'a']);
        $this->assertEquals([[1, 2]], [[1, 2]]);
        $this->assertEquals([new \stdClass()], [1]);
    }

    public function testNumericStrings(): void
    {
        $this->assertEquals('1', 'x');
        $this->assertEquals(' 1.5e3 ', 'x');
        $this->assertEquals('-1.', 'x');
        $this->assertEquals('.5', 'x');
        $this->assertEquals('.', 'x');
        $this->assertEquals('1e', 'x');
        $this->assertEquals('1e+5', 'x');
        $this->assertEquals('1.2.3', 'x');
        $this->assertEquals('abc', 'x');
        $this->assertEquals('', 'x');
        $this->assertEquals(1, 'x');
        $this->assertEquals(1.5, 2);
        $this->assertEquals(true, 'x');
        $this->assertEquals('x', new \stdClass());
        $this->assertEquals('x', new class implements \Stringable {
            public function __toString(): string
            {
                return 'x';
            }
        });
        $this->assertEquals('x', $this->nullable());
        $this->assertEquals('x', $this->union());
    }

    private function nullable(): ?string
    {
        return null;
    }

    private function union(): int|string
    {
        return 1;
    }
}
-----
<?php

namespace App\Tests\SameScalars;

use PHPUnit\Framework\TestCase;

final class SameScalarsTest extends TestCase
{
    private const LIST = ['a' => 1, 'b' => 2];

    public function testArrays(): void
    {
        $this->assertEquals(['a' => 1, 'b' => 2], ['b' => 2, 'a' => 1]);
        $this->assertSame(['a' => 1, 'b' => 2], ['a' => 1, 'b' => 2]);
        $this->assertSame(array(1, 2), array(1, 2));
        $this->assertSame([5 => 'x', 'y'], [5 => 'x', 6 => 'y']);
        $this->assertEquals([1, 2], [1, 3 => 2]);
        $this->assertSame(self::LIST, ['a' => 1, 'b' => 2]);
        $this->assertSame(['a' => 1, 'b' => 2], self::LIST);
        $expected = ['x' => 1, 'y' => 2];
        $actual = ['y' => 2, 'x' => 1];
        $this->assertEquals($expected, $actual);
        $same = ['x' => 1, 'y' => 2];
        $this->assertSame($expected, $same);
        $this->assertSame([1, 2], [1, ...[2]]);
        $this->assertSame([1.5 => 'a'], [1 => 'a']);
        $this->assertSame([true => 'a'], [1 => 'a']);
        $this->assertSame([[1, 2]], [[1, 2]]);
        $this->assertEquals([new \stdClass()], [1]);
    }

    public function testNumericStrings(): void
    {
        $this->assertEquals('1', 'x');
        $this->assertEquals(' 1.5e3 ', 'x');
        $this->assertEquals('-1.', 'x');
        $this->assertEquals('.5', 'x');
        $this->assertSame('.', 'x');
        $this->assertSame('1e', 'x');
        $this->assertEquals('1e+5', 'x');
        $this->assertSame('1.2.3', 'x');
        $this->assertSame('abc', 'x');
        $this->assertSame('', 'x');
        $this->assertEquals(1, 'x');
        $this->assertEquals(1.5, 2);
        $this->assertEquals(true, 'x');
        $this->assertSame('x', new \stdClass());
        $this->assertSame('x', new class implements \Stringable {
            public function __toString(): string
            {
                return 'x';
            }
        });
        $this->assertSame('x', $this->nullable());
        $this->assertEquals('x', $this->union());
    }

    private function nullable(): ?string
    {
        return null;
    }

    private function union(): int|string
    {
        return 1;
    }
}
