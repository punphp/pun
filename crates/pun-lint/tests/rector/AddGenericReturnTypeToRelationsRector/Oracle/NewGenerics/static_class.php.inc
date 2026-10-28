<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class User extends Model
{
    public function staticArg(): HasMany
    {
        return $this->hasMany(static::class);
    }

    public function selfArg(): HasMany
    {
        return $this->hasMany(self::class);
    }
}
-----
<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class User extends Model
{
    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\User, $this>
     */
    public function staticArg(): HasMany
    {
        return $this->hasMany(static::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\User, $this>
     */
    public function selfArg(): HasMany
    {
        return $this->hasMany(self::class);
    }
}
