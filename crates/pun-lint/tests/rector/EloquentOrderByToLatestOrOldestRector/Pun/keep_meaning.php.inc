<?php

// Rector reads the variable part of a concatenation as '', so it takes `$name . 'created_at'` for the default column
// and drops it (Pun keeps the column), and `$x . 'asc'` for an ascending direction; it reads no direction from an
// unpacked argument and drops it. Each would change what the query sorts by; Pun leaves the last two calls.

namespace App;

use Illuminate\Database\Query\Builder;

function run(Builder $query, $name, $x, array $more)
{
    $query->orderBy($name . 'created_at');
    $query->orderBy('x_at', $x . 'asc');
    $query->orderBy('x_at', ...$more);
}
-----
<?php

// Rector reads the variable part of a concatenation as '', so it takes `$name . 'created_at'` for the default column
// and drops it (Pun keeps the column), and `$x . 'asc'` for an ascending direction; it reads no direction from an
// unpacked argument and drops it. Each would change what the query sorts by; Pun leaves the last two calls.

namespace App;

use Illuminate\Database\Query\Builder;

function run(Builder $query, $name, $x, array $more)
{
    $query->oldest($name . 'created_at');
    $query->orderBy('x_at', $x . 'asc');
    $query->orderBy('x_at', ...$more);
}
