<?php

namespace App\Tests\Cyc7;

class FirstTest extends SecondTest
{
    public function __construct()
    {
    }

    public function testIt(): void
    {
        $this->assertTrue(true);
    }
}

class SecondTest extends FirstTest
{
}
