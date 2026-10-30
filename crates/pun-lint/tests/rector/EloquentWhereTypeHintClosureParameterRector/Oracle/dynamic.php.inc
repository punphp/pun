<?php

namespace App;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Model;

class Post extends Model {}

class Comment extends Model {}

function run(Builder $query)
{
    $method = 'where';
    $query->$method(function ($q) {});
}
