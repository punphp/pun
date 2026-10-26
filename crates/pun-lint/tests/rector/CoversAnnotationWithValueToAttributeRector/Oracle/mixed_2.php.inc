<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

trait Helper {}

/**
 * @coversDefaultClass ::helper()
 */
final class SomeTest extends TestCase
{
    /**
     * @covers ::foo
     * @covers \App\Tests\Helper
     * @covers \A::b
     *   more
     */
    public function testFoo()
    {
    }
}

$a = new class extends TestCase {
    /**
     * @covers \Y
     * @covers ::z
     */
    public function testA() {}
};
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

trait Helper {}

#[\PHPUnit\Framework\Attributes\CoversFunction('helper')]
#[\PHPUnit\Framework\Attributes\CoversFunction('foo')]
#[\PHPUnit\Framework\Attributes\CoversTrait(\App\Tests\Helper::class)]
#[\PHPUnit\Framework\Attributes\CoversMethod('\A::b
  more::class', 'b
  more')]
final class SomeTest extends TestCase
{
    public function testFoo()
    {
    }
}

$a = new #[\PHPUnit\Framework\Attributes\CoversClass(\Y::class)] #[\PHPUnit\Framework\Attributes\CoversFunction('z')] class extends TestCase {
    public function testA() {}
};
