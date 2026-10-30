<?php

namespace Pun\Oracle\SafeStrict;

function g(int ...$a): void
{
}

g(a: 1);
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

function g(int ...$a): void
{
}

g(a: 1);
