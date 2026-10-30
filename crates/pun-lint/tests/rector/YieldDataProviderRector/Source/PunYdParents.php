<?php

namespace Pun\Yd;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes\Test;
use PHPUnit\Framework\TestCase;

abstract class Middle extends TestCase
{
    protected $shared;

    #[DataProvider('fromAttribute')]
    public function testA($a)
    {
    }

    /**
     * @dataProvider fromDoc
     */
    public function testB($a)
    {
    }
}

abstract class Upper extends Middle
{
    #[Test]
    public function checks()
    {
    }
}

abstract class BrokenBase extends TestCase
{
    #[DataProvider]
    public function testC($a)
    {
    }
}
