<?php

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Account extends Model {}

class User extends Model
{
    /**
     * @return Relations\HasMany
     */
    public function missing(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return Illuminate\Database\Eloquent\Relations\HasMany
     */
    public function existing(): HasMany
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
     * @return Relations\HasMany
     */
    public function missing(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\Account, $this>
     */
    public function existing(): HasMany
    {
        return $this->hasMany(Account::class);
    }
}
