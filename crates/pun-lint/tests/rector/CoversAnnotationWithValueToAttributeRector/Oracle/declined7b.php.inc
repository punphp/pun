<?php

namespace App\Tests\Declined7;

use PHPUnit\Framework\TestCase;

/** @covers \Foo
@group x */
final class UnstarredTest extends TestCase
{
    /**
     * @covers \Foo
     *   ::bar
     */
    public function testMultiLine(): void
    {
    }
}
-----
<?php

namespace App\Tests\Declined7;

use PHPUnit\Framework\TestCase;

/**
@group x */
#[\PHPUnit\Framework\Attributes\CoversClass(\Foo::class)]
#[\PHPUnit\Framework\Attributes\CoversMethod('\Foo
  ::class', '\Foo
  ::bar')]
final class UnstarredTest extends TestCase
{
    public function testMultiLine(): void
    {
    }
}
