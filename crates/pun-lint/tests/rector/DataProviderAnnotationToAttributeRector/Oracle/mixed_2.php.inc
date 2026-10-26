<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @dataProvider it's
     * @dataProvider a$b"c\d
     * @dataProvider X::self::y
     * @dataProvider A-B::c
     * @dataProvider p1
     * @dataProvider p1
     */
    public function testB()
    {
    }

    /**
     * @dataProvider
     *   p2
     */
    public function testC()
    {
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    #[\PHPUnit\Framework\Attributes\DataProvider("it's")]
    #[\PHPUnit\Framework\Attributes\DataProvider('a$b"c\d')]
    #[\PHPUnit\Framework\Attributes\DataProviderExternal(\X::class, self::y)]
    #[\PHPUnit\Framework\Attributes\DataProviderExternal('\A-B::class', 'c')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p1')]
    public function testB()
    {
    }

    #[\PHPUnit\Framework\Attributes\DataProvider('p2')]
    public function testC()
    {
    }
}
