<?php

namespace App\Tests\Declined7;

use PHPUnit\Framework\TestCase;

final class AnonymousDocTest extends TestCase
{
    public function testIt(): void
    {
        $anonymous = new /** @covers \Foo */ readonly class extends TestCase {
        };
    }
}
