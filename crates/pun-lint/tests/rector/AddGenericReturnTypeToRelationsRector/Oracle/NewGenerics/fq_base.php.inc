<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Account extends Model {}

class User extends Model
{
    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany
     */
    public function accounts(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany<Account,
     *     $this>
     */
    public function spread(): HasMany
    {
        return $this->hasMany(Account::class);
    }
}
-----
<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Account extends Model {}

class User extends Model
{
    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     */
    public function accounts(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany<Account,
     *     $this>
     */
    public function spread(): HasMany
    {
        return $this->hasMany(Account::class);
    }
}
