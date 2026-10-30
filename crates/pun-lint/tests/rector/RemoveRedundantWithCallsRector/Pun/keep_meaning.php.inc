<?php

namespace App;

// Rector writes `$maybe ?? 1 + 2`, which adds 2 to the 1 only, and `$list`, the array in place of its first item.
function run(?int $maybe, array $list)
{
    $q = with($maybe ?? 1) + 2;
    $p = with(...$list);
}
-----
<?php

namespace App;

// Rector writes `$maybe ?? 1 + 2`, which adds 2 to the 1 only, and `$list`, the array in place of its first item.
function run(?int $maybe, array $list)
{
    $q = ($maybe ?? 1) + 2;
    $p = with(...$list);
}
