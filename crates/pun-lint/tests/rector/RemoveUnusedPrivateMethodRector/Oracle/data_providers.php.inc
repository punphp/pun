<?php

namespace Pun\Oracle\UnusedPrivate;

use PHPUnit\Framework\Attributes\DataProvider;

final class DataProviders
{
    /**
     * @dataProvider !weird
     */
    public function testA()
    {
    }

    #[DataProvider(methodName: 'namedProvider')]
    #[\Deprecated]
    public function testB()
    {
    }

    private function namedProvider()
    {
    }

    private function weird()
    {
    }
}
-----
<?php

namespace Pun\Oracle\UnusedPrivate;

use PHPUnit\Framework\Attributes\DataProvider;

final class DataProviders
{
    /**
     * @dataProvider !weird
     */
    public function testA()
    {
    }

    #[DataProvider(methodName: 'namedProvider')]
    #[\Deprecated]
    public function testB()
    {
    }

    private function namedProvider()
    {
    }
}
