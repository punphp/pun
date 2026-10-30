<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Account extends Model {}

class User extends Model
{
    // Pun reads a generic argument only as a name or `$this`; Rector reads any type and rewrites these tags.
    /**
     * @return HasMany<Account<int>, $this>
     */
    public function nested(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany<Account|null, $this>
     */
    public function union(): HasMany
    {
        return $this->hasMany(Account::class);
    }
}
