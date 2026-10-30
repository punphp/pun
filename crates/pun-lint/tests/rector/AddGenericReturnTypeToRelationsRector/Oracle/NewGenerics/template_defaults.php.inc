<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\Pivot;

class Role extends Model {}

/**
 * @template TRelatedModel of Model
 * @template TDeclaringModel of Model
 * @template TPivotModel of Pivot = Pivot
 * @template TAccessor of string = 'pivot'
 *
 * @extends BelongsToMany<TRelatedModel, TDeclaringModel>
 */
class RoleRelation extends BelongsToMany {}

class RolePivot extends Model {}

/**
 * @template TRelatedModel of Model
 * @template TDeclaringModel of Model
 * @template TPivotModel of Model = RolePivot
 *
 * @extends BelongsToMany<TRelatedModel, TDeclaringModel>
 */
class TagRelation extends BelongsToMany {}

class User extends Model
{
    /**
     * @return RoleRelation<Role, $this>
     */
    public function roles(): RoleRelation
    {
        return $this->belongsToMany(Role::class);
    }

    /**
     * @return RoleRelation<Role, $this, Pivot>
     */
    public function pivotRoles(): RoleRelation
    {
        return $this->belongsToMany(Role::class);
    }

    /**
     * @return RoleRelation<Role>
     */
    public function oneRoles(): RoleRelation
    {
        return $this->belongsToMany(Role::class);
    }

    /**
     * @return RoleRelation<Role, $this, int>
     */
    public function intRoles(): RoleRelation
    {
        return $this->belongsToMany(Role::class);
    }

    /**
     * @return TagRelation<Role, $this>
     */
    public function tags(): TagRelation
    {
        return $this->belongsToMany(Role::class);
    }

    public function bareRoles(): RoleRelation
    {
        return $this->belongsToMany(Role::class);
    }
}
-----
<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\Pivot;

class Role extends Model {}

/**
 * @template TRelatedModel of Model
 * @template TDeclaringModel of Model
 * @template TPivotModel of Pivot = Pivot
 * @template TAccessor of string = 'pivot'
 *
 * @extends BelongsToMany<TRelatedModel, TDeclaringModel>
 */
class RoleRelation extends BelongsToMany {}

class RolePivot extends Model {}

/**
 * @template TRelatedModel of Model
 * @template TDeclaringModel of Model
 * @template TPivotModel of Model = RolePivot
 *
 * @extends BelongsToMany<TRelatedModel, TDeclaringModel>
 */
class TagRelation extends BelongsToMany {}

class User extends Model
{
    /**
     * @return RoleRelation<Role, $this>
     */
    public function roles(): RoleRelation
    {
        return $this->belongsToMany(Role::class);
    }

    /**
     * @return RoleRelation<Role, $this, Pivot>
     */
    public function pivotRoles(): RoleRelation
    {
        return $this->belongsToMany(Role::class);
    }

    /**
     * @return \App\Models\RoleRelation<\App\Models\Role, $this, \Illuminate\Database\Eloquent\Relations\Pivot>
     */
    public function oneRoles(): RoleRelation
    {
        return $this->belongsToMany(Role::class);
    }

    /**
     * @return \App\Models\RoleRelation<\App\Models\Role, $this, \Illuminate\Database\Eloquent\Relations\Pivot>
     */
    public function intRoles(): RoleRelation
    {
        return $this->belongsToMany(Role::class);
    }

    /**
     * @return \App\Models\TagRelation<\App\Models\Role, $this, \Illuminate\Database\Eloquent\Relations\Pivot>
     */
    public function tags(): TagRelation
    {
        return $this->belongsToMany(Role::class);
    }

    /**
     * @return \App\Models\RoleRelation<\App\Models\Role, $this, \Illuminate\Database\Eloquent\Relations\Pivot>
     */
    public function bareRoles(): RoleRelation
    {
        return $this->belongsToMany(Role::class);
    }
}
