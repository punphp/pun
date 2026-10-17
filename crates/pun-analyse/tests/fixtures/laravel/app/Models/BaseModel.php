<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

abstract class BaseModel extends Model
{
    protected $primaryKey = 'uuid';

    protected $keyType = 'string';

    public $incrementing = false;

    public $timestamps = false;

    /** @return array<string, string> */
    protected function casts(): array
    {
        return ['score' => 'integer'];
    }
}
