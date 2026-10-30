<?php

namespace Pun\Yd;

use PHPUnit\Framework\TestCase;

$test = new class('x') extends TestCase {
    /**
     * @dataProvider provide
     */
    public function testIt($a)
    {
    }

    /**
     * @return array<int, self>
     */
    public static function provide()
    {
        return [[1]];
    }
};
