<?php

namespace Pun\Oracle\SafeStrict;

function f(int $i): string
{
    return "a" . $i;
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

function f(int $i): string
{
    return "a" . $i;
}
