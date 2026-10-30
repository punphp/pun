<?php

namespace App;

use Illuminate\Database\Eloquent\Attributes\Scope;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Model;

class Sub extends Builder {}

class Base extends Model
{
    public function scopeShared($query) {}
    protected function scopeHidden($query) {}
    private function scopeSecret($query) {}
}

class Post extends Base
{
    function getAAttribute() {}
    #[Scope] function b(Builder $q) {}
    final public function scopeC($query) {}
    public function scopeHidden($query) {}
    public function scopeSecret($query) {}
    public function scopeD(?Builder $query) {}
    public function scopeE(Sub $query) {}
    public function scopeF(\Illuminate\Contracts\Database\Eloquent\Builder $query) {}
    public function scopeG(Unknown $query) {}
    public function scopeH(Builder|int $query) {}
    public function scope() {}
    public function scopeX() {}
    public function scopes($q) {}
    public function scopeé($q) {}
    public function getAttribute($key) {}
    public function getXAttributes() {}
    public function settAttribute() {}
    public function i(): ?Attribute {}
    public function j(): Attribute|int {}
    public function k(): \Illuminate\Database\Eloquent\Casts\Attribute {}
    public function l(): self {}
    public function creating() {}
    public static function scopeM($q) {}
    #[\Illuminate\Database\Eloquent\Attributes\Scope]
    public function n($q) {}
    #[scope]
    public function o($q) {}
    public    function   scopeP($q) {}
    public function scopeQ($q) {}
    public function SCOPEr($q) {}
    public function GetRAttribute() {}
    public function getUrlAttribute(): string {}
}

class Child extends \RectorLaravel\Tests\Rector\ClassMethod\MakeModelAttributesAndScopesProtectedRector\Source\ParentUser
{
    public function scopeActive($query) {}
    public function scopeOther($query) {}
}

trait T
{
    public function scopeT($q) {}
}

interface I
{
    public function scopeI($q);
}

class Plain
{
    public function scopeZ($q) {}
}

new class extends Model {
    public function scopeAnon($q) {}
};

final class F extends Model
{
    private static function scopeY($q) {}
}
-----
<?php

namespace App;

use Illuminate\Database\Eloquent\Attributes\Scope;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Model;

class Sub extends Builder {}

class Base extends Model
{
    protected function scopeShared($query) {}
    protected function scopeHidden($query) {}
    protected function scopeSecret($query) {}
}

class Post extends Base
{
    protected function getAAttribute() {}
    #[Scope] protected function b(Builder $q) {}
    final public function scopeC($query) {}
    protected function scopeHidden($query) {}
    protected function scopeSecret($query) {}
    public function scopeD(?Builder $query) {}
    public function scopeE(Sub $query) {}
    public function scopeF(\Illuminate\Contracts\Database\Eloquent\Builder $query) {}
    public function scopeG(Unknown $query) {}
    public function scopeH(Builder|int $query) {}
    public function scope() {}
    public function scopeX() {}
    public function scopes($q) {}
    public function scopeé($q) {}
    public function getAttribute($key) {}
    public function getXAttributes() {}
    protected function settAttribute() {}
    protected function i(): ?Attribute {}
    public function j(): Attribute|int {}
    protected function k(): \Illuminate\Database\Eloquent\Casts\Attribute {}
    public function l(): self {}
    public function creating() {}
    public static function scopeM($q) {}
    #[\Illuminate\Database\Eloquent\Attributes\Scope]
    protected function n($q) {}
    #[scope]
    protected function o($q) {}
    public    function   scopeP($q) {}
    public function scopeQ($q) {}
    public function SCOPEr($q) {}
    public function GetRAttribute() {}
    protected function getUrlAttribute(): string {}
}

class Child extends \RectorLaravel\Tests\Rector\ClassMethod\MakeModelAttributesAndScopesProtectedRector\Source\ParentUser
{
    public function scopeActive($query) {}
    protected function scopeOther($query) {}
}

trait T
{
    public function scopeT($q) {}
}

interface I
{
    public function scopeI($q);
}

class Plain
{
    public function scopeZ($q) {}
}

new class extends Model {
    public function scopeAnon($q) {}
};

final class F extends Model
{
    private static function scopeY($q) {}
}
