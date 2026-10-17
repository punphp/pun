<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

final class Third extends Model
{
    protected $table = 'third';

    public $timestamps = false;

    protected $casts = self::NO_CASTS;

    private const NO_CASTS = [];
}
