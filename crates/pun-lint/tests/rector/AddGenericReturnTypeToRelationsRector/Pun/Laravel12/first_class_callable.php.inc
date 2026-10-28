<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class User extends Model
{
    // Rector fails on the file: it asks the type of the placeholder's missing value (`getArgs()[0]->value`).
    public function accounts(): HasMany
    {
        return $this->hasMany(...);
    }
}
