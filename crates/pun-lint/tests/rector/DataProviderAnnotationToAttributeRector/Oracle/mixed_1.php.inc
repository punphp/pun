<?php

namespace App\Tests;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes as A;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @dataProvider(foo)
     * @dataProvider
     * @dataProvider   bar baz
     * @dataProvider Other::prov
     * @dataProvider ::qux
     */
    public function testA()
    {
    }

    /**
     * @dataProvider p1
     * @dataProvider p2
     * @dataProvider p3
     */
    #[DataProvider('p1')]
    #[A\DataProvider('p2')]
    #[\PHPUnit\Framework\Attributes\DataProvider("p3")]
    public function testB()
    {
    }

    /**
     * @dataProvider p4
     */
    #[DataProvider('p4'), \Foo]
    public function testC()
    {
    }

    /**
     * @dataProvider \Ext::p5
     * @dataProvider A::B::p6
     * @dataProvider (p7)
     * @DataProvider p8
     * @dataProvider "p9"
     */
    #[\PHPUnit\Framework\Attributes\DataProviderExternal(\Ext::class, 'p5')]
    public function testD()
    {
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes as A;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @dataProvider
     */
    #[\PHPUnit\Framework\Attributes\DataProvider('foo')]
    #[\PHPUnit\Framework\Attributes\DataProvider('bar')]
    #[\PHPUnit\Framework\Attributes\DataProviderExternal(\Other::class, 'prov')]
    #[\PHPUnit\Framework\Attributes\DataProvider('qux')]
    public function testA()
    {
    }

    #[DataProvider('p1')]
    #[A\DataProvider('p2')]
    #[\PHPUnit\Framework\Attributes\DataProvider("p3")]
    #[\PHPUnit\Framework\Attributes\DataProvider('p3')]
    public function testB()
    {
    }

    #[DataProvider('p4'), \Foo]
    #[\PHPUnit\Framework\Attributes\DataProvider('p4')]
    public function testC()
    {
    }

    /**
     * @DataProvider p8
     */
    #[\PHPUnit\Framework\Attributes\DataProviderExternal(\Ext::class, 'p5')]
    #[\PHPUnit\Framework\Attributes\DataProviderExternal(\A::class, B::p6)]
    #[\PHPUnit\Framework\Attributes\DataProvider('p7')]
    #[\PHPUnit\Framework\Attributes\DataProvider('p9')]
    public function testD()
    {
    }
}
