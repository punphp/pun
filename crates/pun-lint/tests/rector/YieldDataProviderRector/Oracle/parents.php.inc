<?php

namespace Pun\Yd;

final class ChildTest extends Upper
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

final class ChildTest extends Upper
{
    private $unused;

    public static function fromAttribute(): \Iterator
    {
        yield [1];
        yield [2];
    }

    public static function fromDoc(): \Iterator
    {
        yield [3];
    }
}
