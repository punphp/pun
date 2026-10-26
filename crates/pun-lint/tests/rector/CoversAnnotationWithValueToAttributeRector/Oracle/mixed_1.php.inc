<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

/**
 * Intro.
 *
 * @coversDefaultClass Foo\Bar
 * @covers ::boot
 * @covers Relative
 * @covers \Foo\Baz
 * @covers \Foo\Baz
 * @covers(x)
 */
final class SomeTest extends TestCase
{
    /**
     * @covers ::foo
     * @covers Relative
     * @covers \Foo\Baz
     * @author me
     */
    public function testFoo()
    {
    }

    /**
     * @covers \Foo\Bar::baz::qux
     */
    public function testBar()
    {
    }
}

final class OtherTest extends TestCase
{
    /**
     * @covers ::fn()
     * @covers Rel
     */
    public function testA()
    {
    }
}

final class ThirdTest extends TestCase
{
    /**
     * @covers Rel
     */
    public function testA()
    {
    }
}

trait SomeTestTrait
{
    /**
     * @covers \X
     */
    public function testT()
    {
    }
}

$a = new class extends TestCase {
    /**
     * @covers \Y
     */
    public function testA() {}
};
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

/**
 * Intro.
 *
 * @covers Relative
 * @covers(x)
 */
#[\PHPUnit\Framework\Attributes\CoversClass(Foo\Bar::class)]
#[\PHPUnit\Framework\Attributes\CoversClass(\Foo\Baz::class)]
#[\PHPUnit\Framework\Attributes\CoversMethod(Foo\Bar::class, 'foo')]
#[\PHPUnit\Framework\Attributes\CoversMethod(\Foo\Bar::class, 'qux')]
final class SomeTest extends TestCase
{
    /**
     * @author me
     */
    public function testFoo()
    {
    }

    public function testBar()
    {
    }
}

#[\PHPUnit\Framework\Attributes\CoversFunction('fn')]
final class OtherTest extends TestCase
{
    public function testA()
    {
    }
}

final class ThirdTest extends TestCase
{
    public function testA()
    {
    }
}

trait SomeTestTrait
{
    public function testT()
    {
    }
}

$a = new #[\PHPUnit\Framework\Attributes\CoversClass(\Y::class)] class extends TestCase {
    public function testA() {}
};
