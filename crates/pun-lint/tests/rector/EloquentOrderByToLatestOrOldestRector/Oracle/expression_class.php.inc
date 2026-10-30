<?php

namespace App;

use Illuminate\Database\Query\Builder;

class Columns
{
    const CREATED = 'created_at';

    final public const UPDATED = 'updated_at';
}

final class FinalColumns
{
    const CREATED = 'created_at';
}

function run(Builder $query, Columns $columns, FinalColumns $final, $unknown)
{
    $query->orderBy($columns::CREATED);
    $query->orderBy($columns::UPDATED);
    $query->orderBy($final::CREATED);
    $class = FinalColumns::class;
    $query->orderBy($class::CREATED);
    $other = Columns::class;
    $query->orderBy($other::CREATED, 'desc');
    $query->orderBy($unknown::CREATED);
}
-----
<?php

namespace App;

use Illuminate\Database\Query\Builder;

class Columns
{
    const CREATED = 'created_at';

    final public const UPDATED = 'updated_at';
}

final class FinalColumns
{
    const CREATED = 'created_at';
}

function run(Builder $query, Columns $columns, FinalColumns $final, $unknown)
{
    $query->orderBy($columns::CREATED);
    $query->oldest($columns::UPDATED);
    $query->oldest();
    $class = FinalColumns::class;
    $query->oldest();
    $other = Columns::class;
    $query->latest();
    $query->orderBy($unknown::CREATED);
}
