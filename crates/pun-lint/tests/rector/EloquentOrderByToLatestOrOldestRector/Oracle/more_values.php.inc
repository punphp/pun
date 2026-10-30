<?php

namespace App;

use Illuminate\Database\Query\Builder;

class Columns
{
    const CREATED = 'created_at';
}

function more(Builder $query, $x, int $count, Columns $columns)
{
    $method = 'orderBy';
    $query->$method('x_at');
    $query->orderBy('x' . null . '_at');
    $query->orderBy('x' . false . '_at');
    $query->orderBy(-$x . '_at');
    $created = 'created_at';
    $query->orderBy($created);
    $none = null;
    $query->orderBy('x' . $none . '_at');
    $yes = true;
    $query->orderBy('x' . $yes . '_at');
    $no = false;
    $query->orderBy('x' . $no . '_at');
    $five = 5;
    $query->orderBy('x' . $five . '_at');
    $query->orderBy('x' . $count . '_at');
}
-----
<?php

namespace App;

use Illuminate\Database\Query\Builder;

class Columns
{
    const CREATED = 'created_at';
}

function more(Builder $query, $x, int $count, Columns $columns)
{
    $method = 'orderBy';
    $query->$method('x_at');
    $query->oldest('x' . null . '_at');
    $query->oldest('x' . false . '_at');
    $query->oldest(-$x . '_at');
    $created = 'created_at';
    $query->oldest();
    $none = null;
    $query->oldest('x' . $none . '_at');
    $yes = true;
    $query->oldest('x' . $yes . '_at');
    $no = false;
    $query->oldest('x' . $no . '_at');
    $five = 5;
    $query->oldest('x' . $five . '_at');
    $query->oldest('x' . $count . '_at');
}
