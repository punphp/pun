<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class User extends Model
{
    /**
     * @return HasMany<static, $this>
     */
    public function staticRelated(): HasMany
    {
        return $this->hasMany(User::class);
    }

    /**
     * @return HasMany<$this, $this>
     */
    public function thisRelated(): HasMany
    {
        return $this->hasMany(User::class);
    }
}

class Child extends User
{
    /**
     * @return HasMany<parent, $this>
     */
    public function parentRelated(): HasMany
    {
        return $this->hasMany(User::class);
    }

    /**
     * @return HasMany<parent, $this>
     */
    public function parentOther(): HasMany
    {
        return $this->hasMany(Child::class);
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
     * @return HasMany<static, $this>
     */
    public function staticRelated(): HasMany
    {
        return $this->hasMany(User::class);
    }

    /**
     * @return HasMany<$this, $this>
     */
    public function thisRelated(): HasMany
    {
        return $this->hasMany(User::class);
    }
}

class Child extends User
{
    /**
     * @return HasMany<parent, $this>
     */
    public function parentRelated(): HasMany
    {
        return $this->hasMany(User::class);
    }

    /**
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Child, $this>
     */
    public function parentOther(): HasMany
    {
        return $this->hasMany(Child::class);
    }
}
