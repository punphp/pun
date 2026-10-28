<?php

namespace App;

use Illuminate\Contracts\Database\Query\Builder as QueryContract;
use Illuminate\Database\Eloquent\Builder as EloquentBuilder;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Query\Builder as QueryBuilder;

class Post extends Model
{
    public function scopeRecent($query)
    {
        return self::where(fn ($q) => $q->where('a', 1));
    }

    public static function own()
    {
        return static::orWhere(function ($q) {
            return $q;
        });
    }
}

class Custom extends Model
{
    public static function where($column)
    {
        return null;
    }
}

class NotModel
{
    public static function where($column)
    {
        return null;
    }
}

function run(QueryContract $query, EloquentBuilder $eloquent, QueryBuilder $base, Post $post, ?QueryBuilder $maybe, $unknown)
{
    Post::orWhere(fn ($q) => $q);
    Post::WHERE(fn (?QueryBuilder $q) => $q);
    Post::where(fn (array|int $q) => $q);
    Post::where(fn (QueryBuilder $q) => $q);
    Post::where(fn (self $q) => $q);
    Post::where(fn () => 1);
    Post::where(fn (&$q, $b = 1) => $q);
    Post::where(static fn (...$q) => $q);
    Post::where(fn (#[\SensitiveParameter] $q) => $q);
    Post::where(column: fn ($q) => $q);
    Post::where('a', fn ($q) => $q);
    Post::where((fn ($q) => $q));
    Post::whereIn(fn ($q) => $q);
    $post::where(fn ($q) => $q);
    Custom::where(fn ($q) => $q);
    NotModel::where(fn ($q) => $q);
    $eloquent->where(fn ($q) => $q);
    $base->orWhere(function ($q) {});
    $maybe->where(fn ($q) => $q);
    $query?->where(fn ($q) => $q);
    $unknown->where(fn ($q) => $q);
    $post->where(fn ($q) => $q);
    Post::query()->where(fn ($q) => $q)->orWhere(fn ($q) => $q);
    $query->where(fn (string $q) => $q);
}
-----
<?php

namespace App;

use Illuminate\Contracts\Database\Query\Builder as QueryContract;
use Illuminate\Database\Eloquent\Builder as EloquentBuilder;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Query\Builder as QueryBuilder;

class Post extends Model
{
    public function scopeRecent($query)
    {
        return self::where(fn (\Illuminate\Contracts\Database\Eloquent\Builder $q) => $q->where('a', 1));
    }

    public static function own()
    {
        return static::orWhere(function (\Illuminate\Contracts\Database\Eloquent\Builder $q) {
            return $q;
        });
    }
}

class Custom extends Model
{
    public static function where($column)
    {
        return null;
    }
}

class NotModel
{
    public static function where($column)
    {
        return null;
    }
}

function run(QueryContract $query, EloquentBuilder $eloquent, QueryBuilder $base, Post $post, ?QueryBuilder $maybe, $unknown)
{
    Post::orWhere(fn (\Illuminate\Contracts\Database\Eloquent\Builder $q) => $q);
    Post::WHERE(fn (\Illuminate\Contracts\Database\Eloquent\Builder $q) => $q);
    Post::where(fn (\Illuminate\Contracts\Database\Eloquent\Builder $q) => $q);
    Post::where(fn (QueryBuilder $q) => $q);
    Post::where(fn (self $q) => $q);
    Post::where(fn () => 1);
    Post::where(fn (\Illuminate\Contracts\Database\Eloquent\Builder &$q, $b = 1) => $q);
    Post::where(static fn (\Illuminate\Contracts\Database\Eloquent\Builder ...$q) => $q);
    Post::where(fn (#[\SensitiveParameter] \Illuminate\Contracts\Database\Eloquent\Builder $q) => $q);
    Post::where(column: fn (\Illuminate\Contracts\Database\Eloquent\Builder $q) => $q);
    Post::where('a', fn ($q) => $q);
    Post::where((fn (\Illuminate\Contracts\Database\Eloquent\Builder $q) => $q));
    Post::whereIn(fn ($q) => $q);
    $post::where(fn (\Illuminate\Contracts\Database\Eloquent\Builder $q) => $q);
    Custom::where(fn ($q) => $q);
    NotModel::where(fn ($q) => $q);
    $eloquent->where(fn (\Illuminate\Contracts\Database\Query\Builder $q) => $q);
    $base->orWhere(function (\Illuminate\Contracts\Database\Query\Builder $q) {});
    $maybe->where(fn (\Illuminate\Contracts\Database\Query\Builder $q) => $q);
    $query?->where(fn ($q) => $q);
    $unknown->where(fn ($q) => $q);
    $post->where(fn ($q) => $q);
    Post::query()->where(fn (\Illuminate\Contracts\Database\Query\Builder $q) => $q)->orWhere(fn (\Illuminate\Contracts\Database\Query\Builder $q) => $q);
    $query->where(fn (\Illuminate\Contracts\Database\Query\Builder $q) => $q);
}
