<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @dataProvider(foo) a b
     * @dataProvider (  foo-bar  )
     * @dataProvider(\Foo\bar)
     * @dataProvider(foo3)
     */
    public function testA()
    {
    }

    /**
     * @dataProvider(foo4)
     *   more
     */
    public function testB()
    {
    }

    /**
     * @dataProvider(foo5)x
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
    #[\PHPUnit\Framework\Attributes\DataProvider('foo)a')]
    #[\PHPUnit\Framework\Attributes\DataProvider('foo-bar')]
    #[\PHPUnit\Framework\Attributes\DataProvider('\Foo\bar')]
    #[\PHPUnit\Framework\Attributes\DataProvider('foo3')]
    public function testA()
    {
    }

    #[\PHPUnit\Framework\Attributes\DataProvider('foo4')]
    public function testB()
    {
    }

    #[\PHPUnit\Framework\Attributes\DataProvider('foo5)x')]
    public function testC()
    {
    }
}
