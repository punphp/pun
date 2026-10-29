<?php

namespace Pun\Oracle\SafeStrict;

function f(array $a): array
{
    return array_map(fn ($x) => $x, $a);
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

function f(array $a): array
{
    return array_map(fn ($x) => $x, $a);
}
