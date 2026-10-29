<?php

namespace Pun\Oracle\SafeStrict;

/** @param numeric-string $a */
function f($a): void
{
}

f("1");
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

/** @param numeric-string $a */
function f($a): void
{
}

f("1");
