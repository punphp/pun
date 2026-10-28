<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class Account extends Model {}

class User extends Model
{
    // Rector reprints this comment with the changed doc below it; Pun leaves a doc after another comment alone.
    /**
     * @return BelongsTo
     */
    public function owner(): BelongsTo
    {
        return $this->belongsTo(Account::class);
    }
}
