<?php

namespace Pun\Oracle\SafeStrict;

function f(int $i): string
{
    return str_repeat("a", $i);
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

function f(int $i): string
{
    return str_repeat("a", $i);
}
