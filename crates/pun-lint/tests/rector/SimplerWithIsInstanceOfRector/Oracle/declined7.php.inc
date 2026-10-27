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
        $anonymous = new /** @covers \Foo */ readonly class extends TestCase {
        };
    }
}
