<?php

namespace App\Tests\Lined;

use PHPUnit\Framework\TestCase;

final class DeepTest extends TestCase
{
    /** Text.
            More text.
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

final class DeepTest extends TestCase
{
    /** Text.
          More text.
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
