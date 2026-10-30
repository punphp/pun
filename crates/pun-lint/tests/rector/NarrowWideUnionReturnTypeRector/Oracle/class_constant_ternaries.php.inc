<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

use Acme\Mixed;
use Acme\Single;
use Acme\Unlisted;

final class LocalLevels
{
    public const LEVEL = 1;
}

function local(int $level): array|string|int
{
    return LocalLevels::LEVEL === $level ? [] : 'none';
}

function vendor_root(int $level): array|string|int
{
    return \VendorRoot::LEVEL === $level ? [] : 'none';
}

function unlisted(int $level): array|string|int
{
    return Unlisted::LEVEL === $level ? [] : 'none';
}

function single(int $level): array|string|int
{
    return Single::LEVEL === $level ? [] : 'none';
}

function mixed_versions(int $level): array|string|int
{
    return Mixed::LEVEL === $level ?: 'none';
}
-----
<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

use Acme\Mixed;
use Acme\Single;
use Acme\Unlisted;

final class LocalLevels
{
    public const LEVEL = 1;
}

function local(int $level): array|string
{
    return LocalLevels::LEVEL === $level ? [] : 'none';
}

function vendor_root(int $level): array|string
{
    return \VendorRoot::LEVEL === $level ? [] : 'none';
}

function unlisted(int $level): array|string
{
    return Unlisted::LEVEL === $level ? [] : 'none';
}

function single(int $level): array|string
{
    return Single::LEVEL === $level ? [] : 'none';
}

function mixed_versions(int $level): string
{
    return Mixed::LEVEL === $level ?: 'none';
}
