<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class Account extends Model {}

class User extends Model
{
    /**
     * @return
     */
    public function bare(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return ?HasMany
     */
    public function nullable(): HasMany
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
     * @return
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     */
    public function bare(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return ?HasMany
     */
    public function nullable(): HasMany
    {
        return $this->hasMany(Account::class);
    }
}
