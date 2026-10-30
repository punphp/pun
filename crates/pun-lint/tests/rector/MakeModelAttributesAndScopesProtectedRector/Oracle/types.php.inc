<?php

namespace App;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Model;

class Sub extends Builder {}

/** @mixin Helper */
class Base extends Model
{
}

class Helper
{
    public function mixed(): void {}
}

class Post extends Base
{
    public function scopeAa(null|false $query) {}
    public function scopeBb(false|Builder|null $query) {}
    public function scopeCc(parent $query) {}
    public function scopeDd(\Illuminate\Database\Eloquent\Builder $query) {}
    public function a(): null|false {}
    public function b(): \Illuminate\Database\Eloquent\Casts\Attribute|null {}
    public function mixed(): Attribute {}
    public function c(): (Attribute&Sub)|null {}
    public function GETaAttribute() {}
    public function get_Attribute() {}
    public function   /* x */  scopeEe(/* q */ $q) {}
}
-----
<?php

namespace App;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Model;

class Sub extends Builder {}

/** @mixin Helper */
class Base extends Model
{
}

class Helper
{
    public function mixed(): void {}
}

class Post extends Base
{
    protected function scopeAa(null|false $query) {}
    protected function scopeBb(false|Builder|null $query) {}
    public function scopeCc(parent $query) {}
    protected function scopeDd(\Illuminate\Database\Eloquent\Builder $query) {}
    protected function a(): null|false {}
    protected function b(): \Illuminate\Database\Eloquent\Casts\Attribute|null {}
    public function mixed(): Attribute {}
    public function c(): (Attribute&Sub)|null {}
    public function GETaAttribute() {}
    protected function get_Attribute() {}
    protected function   /* x */  scopeEe(/* q */ $q) {}
}
