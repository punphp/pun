<?php

namespace Pun\Yd;

use PHPUnit\Framework\Attributes\DataProvider;

final class ChildWithUseTest extends Upper
{
    private $unused;

    public static function fromAttribute(): array
    {
        return [[1], [2]];
    }

    public static function fromDoc()
    {
        return [[3]];
    }
}
-----
<?php

namespace Pun\Yd;

use PHPUnit\Framework\Attributes\DataProvider;

final class ChildWithUseTest extends Upper
{
    private $unused;

    public static function fromAttribute(): \Iterator
    {
        yield [1];
        yield [2];
    }

    public static function fromDoc()
    {
        return [[3]];
    }
}
