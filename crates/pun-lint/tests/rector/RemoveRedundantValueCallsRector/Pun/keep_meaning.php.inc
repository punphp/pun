<?php

namespace App;

// Rector writes `$int ?: 2 * 3`, which multiplies only the 2, and `$list`, the array in place of its first item.
function run(int $int, array $list)
{
    $l = value($int ?: 2) * 3;
    $p = value(...$list);
}
-----
<?php

namespace App;

// Rector writes `$int ?: 2 * 3`, which multiplies only the 2, and `$list`, the array in place of its first item.
function run(int $int, array $list)
{
    $l = ($int ?: 2) * 3;
    $p = value(...$list);
}
