<?php

namespace Pun\Yd;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

final class BrokenOwnTest extends TestCase
{
    #[DataProvider]
    public function testD($a)
    {
    }

    #[DataProvider('provide')]
    public function testE($a)
    {
    }

    public static function provide(): array
    {
        return [[1]];
    }
}
-----
<?php

namespace Pun\Yd;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

final class BrokenOwnTest extends TestCase
{
    #[DataProvider]
    public function testD($a)
    {
    }

    #[DataProvider('provide')]
    public function testE($a)
    {
    }

    public static function provide(): \Iterator
    {
        yield [1];
    }
}
