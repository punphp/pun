<?php

namespace App\Models {

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use App\Models\Account as Acc;
use App\Models as M;
use Shared;
use function strlen;
use function App\Support\{first, last};
use App\Support\{function helper, Thing};

class Account extends Model {}

class User extends Model
{
    /**
     * @return HasMany<Acc, $this>
     */
    public function aliased(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany<M\Account, $this>
     */
    public function aliasPrefix(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany<Shared\Account, $this>
     */
    public function connected(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany<App\Models\Account, $this>
     */
    public function existing(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    public function dynamic(): HasMany
    {
        $method = 'hasMany';

        return $this->$method(Account::class);
    }
}

}
-----
<?php

namespace App\Models {

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use App\Models\Account as Acc;
use App\Models as M;
use Shared;
use function strlen;
use function App\Support\{first, last};
use App\Support\{function helper, Thing};

class Account extends Model {}

class User extends Model
{
    /**
     * @return HasMany<Acc, $this>
     */
    public function aliased(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    /**
     * @return HasMany<M\Account, $this>
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
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
     */
    public function existing(): HasMany
    {
        return $this->hasMany(Account::class);
    }

    public function dynamic(): HasMany
    {
        $method = 'hasMany';

        return $this->$method(Account::class);
    }
}

}
