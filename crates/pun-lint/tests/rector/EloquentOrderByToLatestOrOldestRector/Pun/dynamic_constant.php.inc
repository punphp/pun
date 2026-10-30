<?php

namespace App;

use Illuminate\Database\Query\Builder;

class Columns
{
    const CREATED = 'created_at';
}

function run(Builder $query)
{
    // Rector fails on a constant whose name is an expression (`ValueResolver::resolveClassConstFetch()`); Pun leaves
    // the call alone.
    $query->orderBy(Columns::{'CREATED'});
}
