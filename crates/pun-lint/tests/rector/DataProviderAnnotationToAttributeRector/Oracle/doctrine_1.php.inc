<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @dataProvider( foo )
     * @dataProvider(foo2) trailing
     * @dataProvider("foo3")
     * @dataProvider(name="x")
     */
    public function testA()
    {
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    #[\PHPUnit\Framework\Attributes\DataProvider('foo')]
    #[\PHPUnit\Framework\Attributes\DataProvider('foo2)trailing')]
    #[\PHPUnit\Framework\Attributes\DataProvider('foo3')]
    #[\PHPUnit\Framework\Attributes\DataProvider('name="x"')]
    public function testA()
    {
    }
}
