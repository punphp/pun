<?php

namespace App\Tests\Anno2;

use PHPUnit\Framework\TestCase;
use PHPUnit\Framework\Attributes\Ticket as T;

final class Marker
{
}

final class Anno2Test extends TestCase
{
    /**
     * @dataProvider provideData
     * @ticket 12 $this // a comment
     * @T
     * @Marker
     * @phpstan:type x
     * @covers \stdClass
     * @depends testOther
     * @requires PHP 8.0
     * @testWith [1, 2]
     */
    public function testOne(int $a, int $b): void
    {
    }

    /** @dataProvider provideData
     */
    public function testTwo(int $a, int $b): void
    {
    }

    /**
     * @dataProvider provideData */
    public function testThree(int $a, int $b): void
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
-----
<?php

namespace App\Tests\Anno2;

use PHPUnit\Framework\TestCase;
use PHPUnit\Framework\Attributes\Ticket as T;

final class Marker
{
}

final class Anno2Test extends TestCase
{
    /**
     * @dataProvider provideData
     * @ticket 12 $this // a comment
     * @T
     * @Marker
     * @phpstan:type x
     * @covers \stdClass
     * @depends testOther
     * @testWith [1, 2]
     */
    #[\PHPUnit\Framework\Attributes\RequiresPhp('>= 8.0')]
    public function testOne(int $a, int $b): void
    {
    }

    /** @dataProvider provideData
     */
    public function testTwo(int $a, int $b): void
    {
    }

    /**
     * @dataProvider provideData */
    public function testThree(int $a, int $b): void
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
