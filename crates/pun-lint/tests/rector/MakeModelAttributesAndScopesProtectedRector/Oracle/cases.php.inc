<?php

namespace App;

use Illuminate\Database\Eloquent\Attributes\Scope;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Model;

class Sub extends Builder {}

/** @method static void scopeMagic($q) */
class Base extends Model
{
}

class Post extends Base
{
    final public function scopeCc($query) {}
    public function scopeDd(?Builder $query) {}
    public function scopeEe(Sub $query) {}
    public function scopeFf(\Illuminate\Contracts\Database\Eloquent\Builder $query) {}
    public function scopeGg(Unknown $query) {}
    public function scopeHh(Builder|int $query) {}
    public function scopeII(Builder&Sub $query) {}
    public function scopeJj(self $query) {}
    public function scopeKk(mixed $query) {}
    public function scopeXx() {}
    public function scopeMagic($q) {}
    public function scopeéé($q) {}
    public function scopeAa_($q) {}
    public    function   scopePp($q) {}
    public static function scopeMm($q) {}
    public function scopeLl(...$q) {}
    public function scopeNn(&$q) {}
    public function scopeOo(#[X] $q) {}
    public function creating() {}
    public function m(): static {}
    public function n(): Sub {}
    public function o(): attribute {}
    public function p(): Unknown {}
    public function q(): null|Attribute {}
    public function r(): Attribute|false {}
    public function s(): Attribute|bool {}
    #[\Illuminate\Database\Eloquent\Attributes\Scope, X]
    public function t($q) {}
    #[\Illuminate\Database\Eloquent\Attributes\Scope] public static function u($q) {}
}

trait T
{
    public function scopeTt($q) {}
    public function getTtAttribute() {}
}

interface I
{
    public function scopeIi($q);
}

enum E
{
    public function scopeEe($q) {}
}

class Plain
{
    public function scopeZz($q) {}
}

new class extends Model {
    public function scopeAnon($q) {}
};

final class F extends Model
{
    private static function scopeYy($q) {}
    protected function scopeWw($q) {}
}

abstract class A extends Model
{
    abstract public function scopeQq($q);
    abstract function scopeRr($q);
}
-----
<?php

namespace App;

use Illuminate\Database\Eloquent\Attributes\Scope;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Model;

class Sub extends Builder {}

/** @method static void scopeMagic($q) */
class Base extends Model
{
}

class Post extends Base
{
    final protected function scopeCc($query) {}
    protected function scopeDd(?Builder $query) {}
    protected function scopeEe(Sub $query) {}
    public function scopeFf(\Illuminate\Contracts\Database\Eloquent\Builder $query) {}
    public function scopeGg(Unknown $query) {}
    public function scopeHh(Builder|int $query) {}
    public function scopeII(Builder&Sub $query) {}
    public function scopeJj(self $query) {}
    public function scopeKk(mixed $query) {}
    public function scopeXx() {}
    public function scopeMagic($q) {}
    public function scopeéé($q) {}
    protected function scopeAa_($q) {}
    protected function   scopePp($q) {}
    public static function scopeMm($q) {}
    protected function scopeLl(...$q) {}
    protected function scopeNn(&$q) {}
    protected function scopeOo(#[X] $q) {}
    public function creating() {}
    public function m(): static {}
    public function n(): Sub {}
    protected function o(): attribute {}
    public function p(): Unknown {}
    protected function q(): null|Attribute {}
    protected function r(): Attribute|false {}
    public function s(): Attribute|bool {}
    #[\Illuminate\Database\Eloquent\Attributes\Scope, X]
    protected function t($q) {}
    #[\Illuminate\Database\Eloquent\Attributes\Scope] public static function u($q) {}
}

trait T
{
    protected function scopeTt($q) {}
    protected function getTtAttribute() {}
}

interface I
{
    public function scopeIi($q);
}

enum E
{
    public function scopeEe($q) {}
}

class Plain
{
    public function scopeZz($q) {}
}

new class extends Model {
    public function scopeAnon($q) {}
};

final class F extends Model
{
    private static function scopeYy($q) {}
    protected function scopeWw($q) {}
}

abstract class A extends Model
{
    abstract protected function scopeQq($q);
    abstract protected function scopeRr($q);
}
