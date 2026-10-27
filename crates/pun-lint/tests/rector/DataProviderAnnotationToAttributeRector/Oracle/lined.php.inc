<?php

namespace App\Tests\Lined;

use PHPUnit\Framework\TestCase;

final class LinedTest extends TestCase
{
    /** Text.
        @dataProvider provide
     */
    public function testIt($a): void
    {
    }

    public static function provide(): array
    {
        return [[1]];
    }
}
-----
<?php

namespace App\Tests\Lined;

use PHPUnit\Framework\TestCase;

final class LinedTest extends TestCase
{
    /** Text.
     */
    #[\PHPUnit\Framework\Attributes\DataProvider('provide')]
    public function testIt($a): void
    {
    }

    public static function provide(): array
    {
        return [[1]];
    }
}
