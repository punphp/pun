<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    const A = 1;

    private $a;
    protected $b;

    public function testZero() {}

    /**
     * Text.
     *
     * @testWith ["a/b", "é", 1.5, 1e25, 0.1, true, null, -3, {"k": "v", "1": 2, "it's": 3}]
     * @author me
     * @testWith [1]
     *           [2]
     */
    #[\Foo]
    public function testOne($v)
    {
    }

    /**
     * @testWith [1]
     *           [2]
     */
    public function testTwo($v)
    {
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    const A = 1;

    private $a;
    protected $b;
    public function dataProviderTwo()
    {
        return [[1], [2]];
    }
    public function dataProviderOne()
    {
        return [["a/b", "é", 1.5, 1.0e+25, 0.1, true, null, -3, ['k' => "v", 2, 'it\'s' => 3]]];
    }

    public function testZero() {}

    /**
     * Text.
     *
     * @author me
     * @dataProvider dataProviderOne
     */
    #[\Foo]
    public function testOne($v)
    {
    }

    /**
     * @dataProvider dataProviderTwo
     */
    public function testTwo($v)
    {
    }
}
