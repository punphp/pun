<?php

namespace App\Tests\DpAttr;

use PHPUnit\Framework\Attributes\Group;
use PHPUnit\Framework\TestCase;

final class DpAttrTest extends TestCase
{
    private const NAME = 'x';

    /**
     * @dataProvider provideData
     */
    #[Group]
    #[Group(name: 'x')]
    #[Group(static::NAME)]
    #[Group(self::NAME)]
    #[Group(DpAttrTest::NAME)]
    #[Group(1)]
    public function testExisting(int $a): void
    {
    }

    /**
     * @dataProvider()
     */
    public function testEmpty(int $a): void
    {
    }

    public static function provideData(): iterable
    {
        yield [1];
    }
}
-----
<?php

namespace App\Tests\DpAttr;

use PHPUnit\Framework\Attributes\Group;
use PHPUnit\Framework\TestCase;

final class DpAttrTest extends TestCase
{
    private const NAME = 'x';

    #[Group]
    #[Group(name: 'x')]
    #[Group(static::NAME)]
    #[Group(self::NAME)]
    #[Group(DpAttrTest::NAME)]
    #[Group(1)]
    #[\PHPUnit\Framework\Attributes\DataProvider('provideData')]
    public function testExisting(int $a): void
    {
    }

    #[\PHPUnit\Framework\Attributes\DataProvider('')]
    public function testEmpty(int $a): void
    {
    }

    public static function provideData(): iterable
    {
        yield [1];
    }
}
