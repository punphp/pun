<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * Text.
     *
     * @testWith [1, -2, 1.5, 1.0, 1e25, 0.1, -0.0, 1e-7, 100000000000000000, true, false, null]
     *           ["it's", "a\nb", "\"q\"", "Foo::bar", "Foo::bar::baz", "", "x\\y", "a$b"]
     *           {"a": 1, "0": 2, "5": 3, "-1": 4, "01": 5, "Foo::class": 6, "it's": 7, "a": 8}
     *           "plain"
     *           42
     *           []
     *           {}
     * @testwith [0.30000000000000004, 3.14159265358979, 2.5e+20, 123456789.125]
     * @author me
     */
    public function testA()
    {
    }

    /**
     * @testWith ["x"]
     */
    public function notATest()
    {
    }

    /**
     * @test
     * @testWith ["y"]
     */
    public function third()
    {
    }

    /**
     * @testWith ["z"]
     */
    #[\PHPUnit\Framework\Attributes\Test]
    protected function testProtected()
    {
    }
}

class NotTestCase
{
    /**
     * @testWith [1]
     */
    public function testB()
    {
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * Text.
     *
     * @author me
     */
    #[\PHPUnit\Framework\Attributes\TestWith([1, -2, 1.5, 1.0, 1.0E+25, 0.1, -0.0, 1.0E-7, 100000000000000000, true, false, null])]
    #[\PHPUnit\Framework\Attributes\TestWith(["it's", 'a
b', 'q', Foo::bar, 'Foo::bar::baz', '', 'x\y', 'a$b'])]
    #[\PHPUnit\Framework\Attributes\TestWith(['a' => 8, 2, 3, 4, '01' => 5, Foo::class => 6, "it's" => 7])]
    #[\PHPUnit\Framework\Attributes\TestWith('plain')]
    #[\PHPUnit\Framework\Attributes\TestWith(42)]
    #[\PHPUnit\Framework\Attributes\TestWith([])]
    #[\PHPUnit\Framework\Attributes\TestWith([])]
    #[\PHPUnit\Framework\Attributes\TestWith([0.30000000000000004, 3.14159265358979, 2.5E+20, 123456789.125])]
    public function testA()
    {
    }

    /**
     * @testWith ["x"]
     */
    public function notATest()
    {
    }

    /**
     * @test
     */
    #[\PHPUnit\Framework\Attributes\TestWith(['y'])]
    public function third()
    {
    }

    /**
     * @testWith ["z"]
     */
    #[\PHPUnit\Framework\Attributes\Test]
    protected function testProtected()
    {
    }
}

class NotTestCase
{
    #[\PHPUnit\Framework\Attributes\TestWith([1])]
    public function testB()
    {
    }
}
