<?php

namespace Pun\Yd;

use PHPUnit\Framework\TestCase;

abstract class AbstractProviderTest extends TestCase
{
    /**
     * @dataProvider provide
     * @dataProvider yieldFromNop
     */
    public function testIt($a)
    {
    }

    abstract public static function provide();

    public static function yieldFromNop()
    {
        yield from [[1]];
        // done
    }
}
