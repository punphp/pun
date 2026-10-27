<?php

namespace App\Tests\Cover12;

use PHPUnit\Framework\TestCase;

class Depends
{
}

final class Cover12Test extends TestCase
{
    public function testA(): int
    {
        return 1;
    }

    /**
     * @depends testA
     */
    public function testB($a): void
    {
    }
}
