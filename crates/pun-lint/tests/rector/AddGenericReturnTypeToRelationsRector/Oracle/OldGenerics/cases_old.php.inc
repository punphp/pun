<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\MorphTo;

class Account extends Model {}

trait HasOwner
{
    public function owner(): BelongsTo
    {
        return $this->belongsTo(Account::class);
    }
}

class User extends Model
{
    use HasOwner;

    /**
     * @return hasMany
     */
    public function lowerTag(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    public function lowerClass(): HasMany
    {
        return $this->hasMany(account::class);
    }

    #[\Deprecated]
    public function withAttribute(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    // a comment
    public function withComment(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    public function parent(): BelongsTo
    {
        return $this->belongsTo(self::class);
    }

    public function fq(): \Illuminate\Database\Eloquent\Relations\HasMany
    {
        return $this->hasMany(Account::class);
    }

    public function wrongCase(): HasMany
    {
        return $this->HasMany(Account::class);
    }

    public function nestedFirst(): HasMany
    {
        $f = function () {
            return $this->hasMany(User::class);
        };

        return $this->hasMany(Account::class);
    }

    public function earlyReturn($x): HasMany
    {
        if ($x) {
            return $x->foo();
        }

        return $this->hasMany(Account::class);
    }

    public function morph(): MorphTo
    {
        return $this->morphTo();
    }

    /**
     * @return HasMany the accounts
     */
    public function described(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany<Account>
     */
    public function oldDone(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany<Account, $this>
     */
    public function newStyle(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany|null
     */
    public function nullable(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    public function stringArg(): HasMany
    {
        return $this->hasMany('App\Models\Account');
    }

    public function unknownArg(): HasMany
    {
        return $this->hasMany('App\Models\Missing');
    }

    public function closureArg(): HasMany
    {
        return collect(fn () => $this->hasMany(Account::class))->first();
    }

    public function nullsafe(): HasMany
    {
        return $this?->hasMany(Account::class);
    }

    public function chained(): BelongsTo
    {
        return $this->belongsTo(Account::class)->withDefault();
    }

    /**
     * @return BelongsTo<Account, User>
     */
    public function childDone(): BelongsTo
    {
        return $this->belongsTo(Account::class);
    }

    /**
     * @return BelongsTo<Account, self>
     */
    public function childSelf(): BelongsTo
    {
        return $this->belongsTo(Account::class);
    }

    /**
     * @return BelongsTo<Account>
     */
    public function childMissing(): BelongsTo
    {
        return $this->belongsTo(Account::class);
    }

    /**
     * @return BelongsTo<Account, Account>
     */
    public function childOther(): BelongsTo
    {
        return $this->belongsTo(Account::class);
    }

    /**
     * @return BelongsTo<Account, $this>
     */
    public function childThis(): BelongsTo
    {
        return $this->belongsTo(Account::class);
    }
}

abstract class Base extends Model
{
    abstract public function rel(): HasMany;
}

class Plain
{
    public function accounts(): HasMany
    {
        return $this->hasMany(Account::class);
    }
}

$anon = new class extends Model
{
    public function accounts(): HasMany
    {
        return $this->hasMany(Account::class);
    }
};
-----
<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\MorphTo;

class Account extends Model {}

trait HasOwner
{
    /**
     * @return \Illuminate\Database\Eloquent\Relations\BelongsTo<\App\Models\Account, \App\Models\HasOwner>
     */
    public function owner(): BelongsTo
    {
        return $this->belongsTo(Account::class);
    }
}

class User extends Model
{
    use HasOwner;

    /**
     * @return hasMany
     */
    public function lowerTag(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\account>
     */
    public function lowerClass(): HasMany
    {
        return $this->hasMany(account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account>
     */
    #[\Deprecated]
    public function withAttribute(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    // a comment
    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account>
     */
    public function withComment(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\BelongsTo<\App\Models\User, \App\Models\User>
     */
    public function parent(): BelongsTo
    {
        return $this->belongsTo(self::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account>
     */
    public function fq(): \Illuminate\Database\Eloquent\Relations\HasMany
    {
        return $this->hasMany(Account::class);
    }

    public function wrongCase(): HasMany
    {
        return $this->HasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account>
     */
    public function nestedFirst(): HasMany
    {
        $f = function () {
            return $this->hasMany(User::class);
        };

        return $this->hasMany(Account::class);
    }

    public function earlyReturn($x): HasMany
    {
        if ($x) {
            return $x->foo();
        }

        return $this->hasMany(Account::class);
    }

    public function morph(): MorphTo
    {
        return $this->morphTo();
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account> the accounts
     */
    public function described(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany<Account>
     */
    public function oldDone(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany<Account, $this>
     */
    public function newStyle(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany|null
     */
    public function nullable(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account>
     */
    public function stringArg(): HasMany
    {
        return $this->hasMany('App\Models\Account');
    }

    public function unknownArg(): HasMany
    {
        return $this->hasMany('App\Models\Missing');
    }

    public function closureArg(): HasMany
    {
        return collect(fn () => $this->hasMany(Account::class))->first();
    }

    public function nullsafe(): HasMany
    {
        return $this?->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\BelongsTo<\App\Models\Account, \App\Models\User>
     */
    public function chained(): BelongsTo
    {
        return $this->belongsTo(Account::class)->withDefault();
    }

    /**
     * @return BelongsTo<Account, User>
     */
    public function childDone(): BelongsTo
    {
        return $this->belongsTo(Account::class);
    }

    /**
     * @return BelongsTo<Account, self>
     */
    public function childSelf(): BelongsTo
    {
        return $this->belongsTo(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\BelongsTo<\App\Models\Account, \App\Models\User>
     */
    public function childMissing(): BelongsTo
    {
        return $this->belongsTo(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\BelongsTo<\App\Models\Account, \App\Models\User>
     */
    public function childOther(): BelongsTo
    {
        return $this->belongsTo(Account::class);
    }

    /**
     * @return BelongsTo<Account, $this>
     */
    public function childThis(): BelongsTo
    {
        return $this->belongsTo(Account::class);
    }
}

abstract class Base extends Model
{
    abstract public function rel(): HasMany;
}

class Plain
{
    public function accounts(): HasMany
    {
        return $this->hasMany(Account::class);
    }
}

$anon = new class extends Model
{
    public function accounts(): HasMany
    {
        return $this->hasMany(Account::class);
    }
};
