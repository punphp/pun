<?php

namespace PunEloquentBuilders;

use App\Account;
use App\User;
use Illuminate\Database\Eloquent\Attributes\Scope;
use Illuminate\Database\Eloquent\Attributes\UseEloquentBuilder;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Database\Eloquent\Model;

use function PHPStan\Testing\assertType;

/**
 * @param \Illuminate\Support\Collection<int, ?User> $users
 * @param Collection<int, User> $all
 */
function test(User $user, \Illuminate\Support\Collection $users, Collection $all, string $view): void
{
    assertType('PunEloquentBuilders\\PostBuilder<PunEloquentBuilders\\Post>', Post::where('a', 1));
    assertType('PunEloquentBuilders\\PostBuilder<PunEloquentBuilders\\Post>', Post::query());
    assertType('PunEloquentBuilders\\Post|null', Post::where('a', 1)->published()->first());
    assertType('PunEloquentBuilders\\CommentBuilder<PunEloquentBuilders\\Comment>', Comment::query());
    assertType('PunEloquentBuilders\\CommentBuilder', Comment::orderBy('a'));
    assertType('PunEloquentBuilders\\PostBuilder<PunEloquentBuilders\\Post>', (new Post())->newQuery());
    assertType('Illuminate\\Database\\Eloquent\\Collection<int, PunEloquentBuilders\\Post>', Post::get());
    assertType('PunEloquentBuilders\\PostBuilder<PunEloquentBuilders\\Post>', Post::recent());
    assertType('PunEloquentBuilders\\PostBuilder<PunEloquentBuilders\\Post>', Post::query()->recent());
    assertType('Illuminate\\Database\\Eloquent\\Builder<App\\User>', User::query()->active());
    assertType('Illuminate\\Database\\Eloquent\\Relations\\HasMany<App\\Account, App\\User>', $user->accounts()->active());
    assertType('int', User::query()->restore());
    assertType('Illuminate\\Database\\Eloquent\\Builder<App\\User>', User::query()->withTrashed());
    assertType('Illuminate\\Support\\HigherOrderTapProxy<App\\User>', tap($user));
    assertType('App\\User', tap($user)->save());
    assertType('App\\User', tap($user, function (User $u): void {}));
    assertType('int', $all->sum->id());
    assertType('Illuminate\\Support\\Collection<int, App\\User>', $users->whereNotNull());
    assertType('Illuminate\\Support\\Collection<int, App\\User>', $users->whereNotNull('name'));
    if (view()->exists($view)) {
    }
}

class Post extends Model
{
    /** @return PostBuilder<Post> */
    public function newEloquentBuilder($query): PostBuilder
    {
        return new PostBuilder($query);
    }

    /**
     * @param Builder<static> $query
     * @return Builder<static>
     */
    #[Scope]
    protected function recent(Builder $query): Builder
    {
        return $query;
    }

    public static function make2(): void
    {
        assertType('PunEloquentBuilders\\PostBuilder<static(PunEloquentBuilders\\Post)>', static::query());
    }
}

/**
 * @template TModel of Post
 * @extends Builder<TModel>
 */
class PostBuilder extends Builder
{
    /** @return $this */
    public function published(): static
    {
        assertType('static(PunEloquentBuilders\\PostBuilder<TModel of PunEloquentBuilders\\Post (class PunEloquentBuilders\\PostBuilder, argument)>)', $this->where('a', 1));
        assertType('PunEloquentBuilders\\PostBuilder<TModel of PunEloquentBuilders\\Post (class PunEloquentBuilders\\PostBuilder, argument)>', $this->whereIn('a', [1]));
        return $this;
    }
}

#[UseEloquentBuilder(CommentBuilder::class)]
class Comment extends Model
{
}

/** @extends Builder<Comment> */
class CommentBuilder extends Builder
{
}
