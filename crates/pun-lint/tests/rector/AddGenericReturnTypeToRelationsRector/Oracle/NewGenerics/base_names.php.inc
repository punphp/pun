<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasMany as Many;
use Illuminate\Database\Eloquent\Relations as R;
use Illuminate\Database\Eloquent;
use Relations;

class Account extends Model {}

class User extends Model
{
    /**
     * @return Many
     */
    public function aliased(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return R\HasMany
     */
    public function aliasPrefix(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return Eloquent\Relations\HasMany
     */
    public function connected(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return Relations\HasMany
     */
    public function sameConnected(): HasMany
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

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasMany as Many;
use Illuminate\Database\Eloquent\Relations as R;
use Illuminate\Database\Eloquent;
use Relations;

class Account extends Model {}

class User extends Model
{
    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     */
    public function aliased(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     */
    public function aliasPrefix(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     */
    public function connected(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return Relations\HasMany
     */
    public function sameConnected(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     */
    public function existing(): HasMany
    {
        return $this->hasMany(Account::class);
    }
}
