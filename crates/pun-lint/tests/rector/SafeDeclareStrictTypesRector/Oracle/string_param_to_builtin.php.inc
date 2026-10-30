<?php

namespace Pun\Oracle\SafeStrict;

function f(string $x): int
{
    return strlen($x);
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

function f(string $x): int
{
    return strlen($x);
}
