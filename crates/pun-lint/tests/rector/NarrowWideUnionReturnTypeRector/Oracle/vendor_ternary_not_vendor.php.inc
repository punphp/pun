<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

final class LocalConstants
{
    public const API = 2;

    public function run(int $level): array|string|int
    {
        return self::API === 2 ? [$level] : 'x';
    }
}
-----
<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

final class LocalConstants
{
    public const API = 2;

    public function run(int $level): array
    {
        return self::API === 2 ? [$level] : 'x';
    }
}
