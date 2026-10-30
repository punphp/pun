<?php

namespace App;

use Illuminate\Database\Eloquent\Builder;

function run(Builder $query)
{
    $method = 'whereHas';
    $query->$method('posts', function ($q) {});
}
