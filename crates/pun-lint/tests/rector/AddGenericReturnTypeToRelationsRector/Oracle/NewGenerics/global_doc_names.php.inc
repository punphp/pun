<?php

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Account extends Model {}

class User extends Model
{
    /**
     * @return HasMany<Missing\Account, $this>
     */
    public function missing(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany<Account, $this>
     */
    public function accounts(): HasMany
    {
        return $this->hasMany(Account::class);
    }
}
-----
<?php

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Account extends Model {}

class User extends Model
{
    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\Account, $this>
     */
    public function missing(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany<Account, $this>
     */
    public function accounts(): HasMany
    {
        return $this->hasMany(Account::class);
    }
}
