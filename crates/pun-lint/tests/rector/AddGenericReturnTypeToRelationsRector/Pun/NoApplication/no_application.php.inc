<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Account extends Model {}

// Without `Illuminate\Foundation\Application`, Rector's `ApplicationAnalyzer` cannot read `VERSION` and fails on the
// file; Pun leaves the method alone.
class User extends Model
{
    public function accounts(): HasMany
    {
        return $this->hasMany(Account::class);
    }
}
