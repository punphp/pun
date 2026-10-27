<?php

namespace App\Tests\Anno;

use PHPUnit\Framework\TestCase;

/**
 * @coversDefaultClass(\stdClass)
 * @covers(\stdClass)
 * @ticket(1)
 * @requires(PHP 8.0)
 */
final class ParenthesizedTest extends TestCase
{
    /**
     * @covers(\stdClass)
     * @testWith([1, 2])
     * @dataProvider(provideData)
     * @depends(testOther)
     * @ticket(1)
     * @requires(PHP 8.0)
     * @test(x)
     * @backupGlobals(enabled)
     */
    public function testParenthesized(int $a, int $b): void
    {
    }

    public static function provideData(): iterable
    {
        yield [1, 2];
    }

    public function testOther(): int
    {
        return 1;
    }
}

/**
 * @testWith [1]
 */
final class NotATest
{
    /**
     * @testWith [1]
     * @covers \stdClass
     */
    public function testIt(int $a): void
    {
    }
}

final class PlainTest extends TestCase
{
    public function testIt(): void
    {
    }
}
-----
<?php

namespace App\Tests\Anno;

use PHPUnit\Framework\TestCase;

/**
 * @coversDefaultClass(\stdClass)
 * @covers(\stdClass)
 * @ticket(1)
 * @requires(PHP 8.0)
 */
final class ParenthesizedTest extends TestCase
{
    /**
     * @covers(\stdClass)
     * @testWith([1, 2])
     * @dataProvider(provideData)
     * @depends(testOther)
     * @ticket(1)
     * @requires(PHP 8.0)
     * @test(x)
     * @backupGlobals(enabled)
     */
    public function testParenthesized(int $a, int $b): void
    {
    }

    public static function provideData(): iterable
    {
        yield [1, 2];
    }

    public function testOther(): int
    {
        return 1;
    }
}

/**
 * @testWith [1]
 */
final class NotATest
{
    /**
     * @covers \stdClass
     */
    #[\PHPUnit\Framework\Attributes\TestWith([1])]
    public function testIt(int $a): void
    {
    }
}

final class PlainTest extends TestCase
{
    public function testIt(): void
    {
    }
}
