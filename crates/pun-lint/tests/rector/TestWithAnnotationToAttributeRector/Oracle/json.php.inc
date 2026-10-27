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
    #[\PHPUnit\Framework\Attributes\TestWith(['ab', ['k' => '', 2, '01' => 3, 4]])]
    #[\PHPUnit\Framework\Attributes\TestWith([null, true, 1.5E-7, 2.0E+20])]
    public function testJson($a, $b): void
    {
    }
}
