<?php

namespace App\PunCollections;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Writer extends Model
{
    protected $table = 'users';

    /** @return HasMany<Writer, $this> */
    public function friends(): HasMany
    {
        return $this->hasMany(Writer::class);
    }

    /** @return \Illuminate\Database\Eloquent\Collection<int, static> */
    public static function everyone()
    {
        return static::all();
    }

    public function countAll(): int
    {
        return static::all()->count();
    }

    public function getEmailAttribute(): string
    {
        return 'email';
    }
}
