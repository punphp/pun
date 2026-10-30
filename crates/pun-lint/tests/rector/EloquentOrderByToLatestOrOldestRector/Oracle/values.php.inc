<?php

namespace App;

use Illuminate\Database\Query\Builder;

function run(Builder $query, $col, $x, $updated_at)
{
    $query->orderBy('x_at', 'desc', 'extra');
    $query->orderBy('x_at', 'desc',);
    $query->orderBy('x_at' , 'desc');
    $query->orderBy('x_at', 'desc' /* d */);
    $query->orderBy('x_at', /* d */ 'desc');
    $query->orderBy(direction: 'asc', column: 'x_at',);
    $query->orderBy(
        direction: 'asc',
        column: 'x_at',
    );
    $query->ORDERBYDESC('created_on');
    $query->orderBy('a_date');
    $query->orderBy('x_At');
    $query->orderBy($col);
    $query->orderBy($updated_at);
    $query->orderBy($x . '_at');
    $query->orderBy('x_at', 'de' . 'sc');
    $query->orderBy('x' . 1 . '_at');
    $query->orderBy(1.5 . '_at');
    $query->orderBy(true . '_at');
    $query->orderBy("{$x}_at");
    $query->orderBy(<<<EOT
    x_at
    EOT);
    $query->orderBy('x_at', ASC);
    $query->orderBy('x_at', \Foo\ASC);
    $query->orderBy(\PDO::class . '_at');
}
-----
<?php

namespace App;

use Illuminate\Database\Query\Builder;

function run(Builder $query, $col, $x, $updated_at)
{
    $query->latest('x_at');
    $query->latest('x_at',);
    $query->latest('x_at');
    $query->latest('x_at' /* d */);
    $query->latest('x_at');
    $query->oldest(column: 'x_at',);
    $query->oldest(
        column: 'x_at',
    );
    $query->latest('created_on');
    $query->oldest('a_date');
    $query->orderBy('x_At');
    $query->orderBy($col);
    $query->oldest($updated_at);
    $query->oldest($x . '_at');
    $query->latest('x_at');
    $query->oldest('x' . 1 . '_at');
    $query->oldest(1.5 . '_at');
    $query->oldest(true . '_at');
    $query->orderBy("{$x}_at");
    $query->oldest(<<<EOT
    x_at
    EOT);
    $query->oldest('x_at');
    $query->latest('x_at');
    $query->oldest(\PDO::class . '_at');
}
