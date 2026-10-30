<?php

namespace Pun\Oracle\SafeStrict;

/** @param string $a */
function f($a): void
{
}

f("a");
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

/** @param string $a */
function f($a): void
{
}

f("a");
