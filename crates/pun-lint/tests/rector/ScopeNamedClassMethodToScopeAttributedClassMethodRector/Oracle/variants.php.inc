<?php

namespace App;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;

trait HasDraft
{
    public function draft() {}
}

/**
 * @method static Builder published()
 */
class A extends Model
{
    use HasDraft;

    /**
     * Doc.
     */
    #[Foo]
    final public static function scopeActive(?Builder $query)
    {
    }

    function scopeB($q) {}

    private function scopeCreating($q) {}

    protected function scopeX(string|Builder $q) {}

    public function scopeTable($q) {}

    public function scopeQuery($q) {}

    public function scopeFooBar(\Illuminate\Database\Query\Builder $q) {}

    public function scopeFoo2() {}

    public function scopeBuilt(Builder&\Countable $q) {}

    function scopeBare(Builder|null $q) {}

    public function scopeDraft($q) {}

    public function scopePublished($q) {}

    public function scopeTouch($q) {}

    public function scopeSelf(self $q) {}

    public function scopeÄrger($q) {}

    public function scopeUnion(Builder|\Countable $q) {}

    public function scopeNullable(?\Countable $q) {}

    #[A, B] public function scopeInline($q) {}
}

class Plain
{
    public function scopeActive($q) {}
}

$m = new class extends Model {
    public function scopeActive($q) {}
};
-----
<?php

namespace App;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;

trait HasDraft
{
    public function draft() {}
}

/**
 * @method static Builder published()
 */
class A extends Model
{
    use HasDraft;

    /**
     * Doc.
     */
    #[Foo]
    #[\Illuminate\Database\Eloquent\Attributes\Scope]
    protected function active(?Builder $query)
    {
    }

    function scopeB($q) {}

    private function scopeCreating($q) {}

    protected function scopeX(string|Builder $q) {}

    #[\Illuminate\Database\Eloquent\Attributes\Scope]
    protected function table($q) {}

    public function scopeQuery($q) {}

    public function scopeFooBar(\Illuminate\Database\Query\Builder $q) {}

    public function scopeFoo2() {}

    public function scopeBuilt(Builder&\Countable $q) {}

    #[\Illuminate\Database\Eloquent\Attributes\Scope]
    protected function bare(Builder|null $q) {}

    public function scopeDraft($q) {}

    public function scopePublished($q) {}

    public function scopeTouch($q) {}

    public function scopeSelf(self $q) {}

    public function scopeÄrger($q) {}

    public function scopeUnion(Builder|\Countable $q) {}

    public function scopeNullable(?\Countable $q) {}

    #[A, B]
    #[\Illuminate\Database\Eloquent\Attributes\Scope] protected function inline($q) {}
}

class Plain
{
    public function scopeActive($q) {}
}

$m = new class extends Model {
    #[\Illuminate\Database\Eloquent\Attributes\Scope]
    protected function active($q) {}
};
