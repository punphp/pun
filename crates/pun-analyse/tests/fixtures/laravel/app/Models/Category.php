<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

final class Category extends Model
{
    public $incrementing = true;

    protected $guarded = [];

    protected $casts = array('position' => 'int');
}
