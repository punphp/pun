<?php

namespace App\Tests\DpAttr;

use PHPUnit\Framework\Attributes\Group;
use PHPUnit\Framework\TestCase;

final class DoctrineTest extends TestCase
{
    /**
     * @dataProvider(provide data)
     */
    public function testNotSimple(int $a): void
    {
    }

    /**
     * @dataProvider(provideData) @other
     */
    public function testAt(int $a): void
    {
    }

    public static function provideData(): iterable
    {
        yield [1];
    }
}
