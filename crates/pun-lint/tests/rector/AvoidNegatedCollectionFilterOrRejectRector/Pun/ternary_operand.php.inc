<?php

namespace App;

use Illuminate\Support\Collection;

// Rector prints the cast without the parentheses its operand needs: `(bool) $x ?: $flag` casts `$x` only.
function run(Collection $items, bool $flag)
{
    $items->FILTER(static fn ($x) => ! ($x ?: $flag));
}
-----
<?php

namespace App;

use Illuminate\Support\Collection;

// Rector prints the cast without the parentheses its operand needs: `(bool) $x ?: $flag` casts `$x` only.
function run(Collection $items, bool $flag)
{
    $items->reject(static fn ($x): bool => (bool) ($x ?: $flag));
}
