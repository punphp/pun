<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    use SomeTrait;

    /**
     * First.
     */
    public function testZero() {}

    /**
     * @testWith [1e16, 1e17, 1.0, 0.00001, 123.456, "a\"b\\c", "\t", "x/y", {"0": 1, "01": 2, "-5": 3}]
     */
    public function testOne($v)
    {
    }

    /**
     * @testWith {"a": [1, {"b": null}]}
     */
    public function it_works($v)
    {
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    use SomeTrait;
    public function dataProviderIt_works()
    {
        return [['a' => [1, ['b' => null]]]];
    }
    public function dataProviderOne()
    {
        return [[10000000000000000.0, 1.0e+17, 1.0, 1.0e-5, 123.456, "a\"b\\c", "\t", "x/y", [1, '01' => 2, 3]]];
    }

    /**
     * First.
     */
    public function testZero() {}

    /**
     * @dataProvider dataProviderOne
     */
    public function testOne($v)
    {
    }

    /**
     * @dataProvider dataProviderIt_works
     */
    public function it_works($v)
    {
    }
}
