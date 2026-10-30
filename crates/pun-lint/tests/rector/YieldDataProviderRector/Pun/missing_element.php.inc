<?php

namespace Pun\Yd;

use PHPUnit\Framework\TestCase;

final class MissingTest extends TestCase
{
    /**
     * @dataProvider provide
     */
    public function testIt($a)
    {
    }

    public static function provide()
    {
        return [, [1]];
    }
}
