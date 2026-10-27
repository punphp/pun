<?php

namespace App\Tests\Unnamed7;

use PHPUnit\Framework\TestCase;

final class UnnamedTest extends TestCase
{
    public function testIt($a, $b): void
    {
        $this->assertTrue($a instanceof $b[0]);
    }
}
