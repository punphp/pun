<?php

namespace App\Tests\TestWithJson;

use PHPUnit\Framework\TestCase;

final class TestWithJsonTest extends TestCase
{
    /**
     * @testWith ["a\rb", {"k": "\r", "1": 2, "01": 3, "-1": 4}]
     *           [null, true, 1.5e-7, 2e20]
     */
    public function testJson($a, $b): void
    {
    }
}
-----
<?php

namespace App\Tests\TestWithJson;

use PHPUnit\Framework\TestCase;

final class TestWithJsonTest extends TestCase
{
    public function dataProviderJson()
    {
        return [["a\rb", ['k' => "\r", 2, '01' => 3, 4]], [null, true, 1.5e-7, 2.0e+20]];
    }
    /**
     * @dataProvider dataProviderJson
     */
    public function testJson($a, $b): void
    {
    }
}
