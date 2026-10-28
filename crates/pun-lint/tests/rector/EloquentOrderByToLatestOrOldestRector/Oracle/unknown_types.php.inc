<?php

namespace App;

use Illuminate\Database\Query\Builder;

class Columns
{
    const CREATED = 'created_at';
}

final class FinalColumns
{
    const CREATED = 'created_at';
}

function run(Builder $query, Columns|FinalColumns $either, FinalColumns $final)
{
    $query->orderBy($either::CREATED);
    $query->orderBy($final::{'CREATED'});
}
