<?php

namespace App\PunRelations;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasOne;

/**
 * @method static void magic()
 */
class Author extends Model
{
    /** @return HasMany<Book, $this> */
    public function books(): HasMany
    {
        return $this->hasMany(Book::class);
    }

    /** @return HasOne<Book, $this> */
    public function latestBook(): HasOne
    {
        return $this->hasOne(Book::class);
    }

    public function untyped()
    {
        return $this->hasMany(Book::class);
    }

    public function name(): string
    {
        return 'name';
    }
}
