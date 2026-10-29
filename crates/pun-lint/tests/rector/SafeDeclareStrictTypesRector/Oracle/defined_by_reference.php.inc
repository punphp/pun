<?php

namespace Pun\Oracle\SafeStrict;

function f(string $s): void
{
    $m = [];
    preg_match("/a/", $s, $m);
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

function f(string $s): void
{
    $m = [];
    preg_match("/a/", $s, $m);
}
