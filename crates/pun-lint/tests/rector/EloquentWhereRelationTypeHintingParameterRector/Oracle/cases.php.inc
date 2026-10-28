<?php

namespace App;

use Illuminate\Contracts\Database\Query\Builder as QueryContract;
use Illuminate\Database\Eloquent\Builder as EloquentBuilder;
use Illuminate\Database\Eloquent\Model;

class Post extends Model
{
}

function run(QueryContract $query, EloquentBuilder $eloquent, Post $post, $unknown)
{
    Post::whereHas('a', fn ($q) => $q);
    Post::orWhereHas('a', function ($q) {});
    Post::whereDoesntHave('a', fn (?EloquentBuilder $q) => $q);
    Post::orWhereDoesntHave('a', fn (EloquentBuilder $q) => $q);
    Post::whereHasMorph('a', ['b'], fn ($q, $type) => $q);
    Post::orWhereHasMorph('a', fn ($q) => $q);
    Post::whereDoesntHaveMorph('a', 'b', fn () => 1);
    Post::orWhereDoesntHaveMorph('a', 'b', static fn (int $q) => $q);
    Post::wherehas('a', fn ($q) => $q);
    Post::whereHas(fn ($q) => $q);
    Post::whereHas('a', 'b', fn ($q) => $q);
    Post::whereRelation('a', fn ($q) => $q);
    $eloquent->whereHas('a', fn ($q) => $q)->orWhereHas('b', fn ($q) => $q);
    $query->whereDoesntHave('a', (fn ($q) => $q));
    $eloquent->whereHasMorph('a', callback: fn ($q) => $q, types: []);
    $unknown->whereHas('a', fn ($q) => $q);
    $post->whereHas('a', fn ($q) => $q);
}
-----
<?php

namespace App;

use Illuminate\Contracts\Database\Query\Builder as QueryContract;
use Illuminate\Database\Eloquent\Builder as EloquentBuilder;
use Illuminate\Database\Eloquent\Model;

class Post extends Model
{
}

function run(QueryContract $query, EloquentBuilder $eloquent, Post $post, $unknown)
{
    Post::whereHas('a', fn (\Illuminate\Contracts\Database\Query\Builder $q) => $q);
    Post::orWhereHas('a', function (\Illuminate\Contracts\Database\Query\Builder $q) {});
    Post::whereDoesntHave('a', fn (\Illuminate\Contracts\Database\Query\Builder $q) => $q);
    Post::orWhereDoesntHave('a', fn (EloquentBuilder $q) => $q);
    Post::whereHasMorph('a', ['b'], fn (\Illuminate\Contracts\Database\Query\Builder $q, $type) => $q);
    Post::orWhereHasMorph('a', fn ($q) => $q);
    Post::whereDoesntHaveMorph('a', 'b', fn () => 1);
    Post::orWhereDoesntHaveMorph('a', 'b', static fn (\Illuminate\Contracts\Database\Query\Builder $q) => $q);
    Post::wherehas('a', fn (\Illuminate\Contracts\Database\Query\Builder $q) => $q);
    Post::whereHas(fn ($q) => $q);
    Post::whereHas('a', 'b', fn ($q) => $q);
    Post::whereRelation('a', fn ($q) => $q);
    $eloquent->whereHas('a', fn (\Illuminate\Contracts\Database\Query\Builder $q) => $q)->orWhereHas('b', fn ($q) => $q);
    $query->whereDoesntHave('a', (fn (\Illuminate\Contracts\Database\Query\Builder $q) => $q));
    $eloquent->whereHasMorph('a', callback: fn ($q) => $q, types: []);
    $unknown->whereHas('a', fn ($q) => $q);
    $post->whereHas('a', fn ($q) => $q);
}
