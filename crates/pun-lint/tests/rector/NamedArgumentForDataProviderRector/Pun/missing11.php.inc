<?php

namespace App\Tests\Missing11;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

final class MissingTest extends TestCase
{
    #[DataProvider('provide')]
    public function testIt(int $first, int $second): void
    {
    }

    public static function provide(): array
    {
        return [[, 1], , [1, 2]];
    }
}
