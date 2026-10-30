<?php

namespace Pun\Oracle\SafeStrict;

function f(int $i): float
{
    return $i;
}

function g(): float
{
    return 1;
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

function f(int $i): float
{
    return $i;
}

function g(): float
{
    return 1;
}
