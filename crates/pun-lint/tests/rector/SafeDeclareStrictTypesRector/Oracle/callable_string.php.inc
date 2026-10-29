<?php

namespace Pun\Oracle\SafeStrict;

function g(callable $c): void
{
}

g("strlen");
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

function g(callable $c): void
{
}

g("strlen");
