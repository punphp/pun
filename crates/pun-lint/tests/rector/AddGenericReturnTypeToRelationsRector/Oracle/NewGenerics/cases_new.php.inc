<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasManyThrough;
use Illuminate\Database\Eloquent\Relations\HasOneThrough;
use Illuminate\Database\Eloquent\Relations\HasOne as One;
use Illuminate\Database\Eloquent\Relations\MorphToMany;
use Illuminate\Database\Eloquent\Relations\Pivot;

class Account extends Model {}

class Car extends Model {}

class User extends Model
{
    /** @var class-string<Account> */
    protected $related = Account::class;

    /**
     * @return HasMany<User>
     */
    public function shortGenericOther(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<User>
     */
    public function fqGenericOther(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return One
     */
    public function aliased(): One
    {
        return $this->hasOne(Account::class);
    }

    /**
     * @return Relations\HasMany
     */
    public function partial(): Relations\HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\BelongsToMany<Account, $this, Account>
     */
    public function notPivot(): BelongsToMany
    {
        return $this->belongsToMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasManyThrough<Account, Car, $this>
     */
    public function throughDone(): HasManyThrough
    {
        return $this->hasManyThrough(Account::class, Car::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasManyThrough<Account, User, $this>
     */
    public function throughOther(): HasManyThrough
    {
        return $this->hasManyThrough(Account::class, Car::class);
    }

    public function throughOne(): HasManyThrough
    {
        return $this->hasManyThrough(Account::class);
    }

    public function morphMany(): MorphToMany
    {
        return $this->morphToMany(Account::class, 'taggable');
    }

    public function staticArg(): HasMany
    {
        return $this->hasMany(static::class);
    }

    public function propertyArg(): HasMany
    {
        return $this->hasMany($this->related);
    }

    /**
     * @return HasMany Many accounts.
     */
    public function described(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /** @return HasMany */
    public function oneLine(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /** Accounts. */
    public function oneLineText(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany
     * @return int
     */
    public function twoTags(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @phpstan-return HasMany<Account, $this>
     */
    public function phpstanTag(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<Account, static>
     */
    public function staticGeneric(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<self, $this>
     */
    public function selfGeneric(): HasMany
    {
        return $this->hasMany(User::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     */
    public function fqArg(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasOneThrough<Account, Car, $this>
     */
    public function oneThroughDone(): HasOneThrough
    {
        return $this->hasOneThrough(Account::class, Car::class);
    }

    /**
     * @return HasOneThrough<Account, User, $this>
     */
    public function oneThroughOther(): HasOneThrough
    {
        return $this->hasOneThrough(Account::class, Car::class);
    }

    public function oneThroughOneArg(): HasOneThrough
    {
        return $this->hasOneThrough(Account::class);
    }

    public function oneThroughUnknown(): HasOneThrough
    {
        return $this->hasOneThrough(Account::class, 'App\Models\Missing');
    }
}
-----
<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasManyThrough;
use Illuminate\Database\Eloquent\Relations\HasOneThrough;
use Illuminate\Database\Eloquent\Relations\HasOne as One;
use Illuminate\Database\Eloquent\Relations\MorphToMany;
use Illuminate\Database\Eloquent\Relations\Pivot;

class Account extends Model {}

class Car extends Model {}

class User extends Model
{
    /** @var class-string<Account> */
    protected $related = Account::class;

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     */
    public function shortGenericOther(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     */
    public function fqGenericOther(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return One
     */
    public function aliased(): One
    {
        return $this->hasOne(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     */
    public function partial(): Relations\HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\BelongsToMany<\App\Models\Account, $this, \Illuminate\Database\Eloquent\Relations\Pivot>
     */
    public function notPivot(): BelongsToMany
    {
        return $this->belongsToMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasManyThrough<Account, Car, $this>
     */
    public function throughDone(): HasManyThrough
    {
        return $this->hasManyThrough(Account::class, Car::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasManyThrough<Account, User, $this>
     */
    public function throughOther(): HasManyThrough
    {
        return $this->hasManyThrough(Account::class, Car::class);
    }

    public function throughOne(): HasManyThrough
    {
        return $this->hasManyThrough(Account::class);
    }

    public function morphMany(): MorphToMany
    {
        return $this->morphToMany(Account::class, 'taggable');
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\User, $this>
     */
    public function staticArg(): HasMany
    {
        return $this->hasMany(static::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     */
    public function propertyArg(): HasMany
    {
        return $this->hasMany($this->related);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this> Many accounts.
     */
    public function described(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /** @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this> */
    public function oneLine(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /** Accounts.
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this> */
    public function oneLineText(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     * @return int
     */
    public function twoTags(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @phpstan-return HasMany<Account, $this>
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     */
    public function phpstanTag(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     */
    public function staticGeneric(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<self, $this>
     */
    public function selfGeneric(): HasMany
    {
        return $this->hasMany(User::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     */
    public function fqArg(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasOneThrough<Account, Car, $this>
     */
    public function oneThroughDone(): HasOneThrough
    {
        return $this->hasOneThrough(Account::class, Car::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasOneThrough<\App\Models\Account, \App\Models\Car, $this>
     */
    public function oneThroughOther(): HasOneThrough
    {
        return $this->hasOneThrough(Account::class, Car::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasOneThrough<\App\Models\Account, $this>
     */
    public function oneThroughOneArg(): HasOneThrough
    {
        return $this->hasOneThrough(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasOneThrough<\App\Models\Account, $this>
     */
    public function oneThroughUnknown(): HasOneThrough
    {
        return $this->hasOneThrough(Account::class, 'App\Models\Missing');
    }
}
