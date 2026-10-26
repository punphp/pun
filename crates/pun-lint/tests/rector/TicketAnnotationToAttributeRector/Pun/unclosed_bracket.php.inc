<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @ticket 1
     * @group x(
     * @param int $a
     * @ticket 2
     * @ticket 3)
     */
    public function testA()
    {
    }
}
