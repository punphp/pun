<?php

namespace Pun\Yd;

final class BrokenChildTest extends BrokenBase
{
    #[\PHPUnit\Framework\Attributes\DataProvider('provide')]
    public function testD($a)
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

final class BrokenChildTest extends BrokenBase
{
    #[\PHPUnit\Framework\Attributes\DataProvider('provide')]
    public function testD($a)
    {
    }

    public static function provide(): \Iterator
    {
        yield [1];
    }
}
