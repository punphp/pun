<?php

namespace App;

use Illuminate\Database\Query\Builder;

function run(Builder $query, $column)
{
    $query->orderBy('name');
    $query->orderByDesc('position');
    $query->orderBy($column, 'desc');
}
-----
<?php

namespace App;

use Illuminate\Database\Query\Builder;

function run(Builder $query, $column)
{
    $query->oldest('name');
    $query->latest('position');
    $query->latest($column);
}
