<?php

namespace App;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Model;

trait HasThing
{
    public function thing() {}
}

class Post extends Model
{
    use HasThing;

    public static function own() {}

    protected function scopePublished(Builder $q) {}

    public function scopeRecent(Builder $q) {}
}

class Plain
{
    public static function where() {}
}

function run(Post $post, string $name, $unknown)
{
    Post::where('a', 1);
    Post::where(
        'a',
        1
    );
    Post::first();
    Post::find(1);
    Post::published();
    Post::recent();
    Post::Recent();
    Post::own();
    Post::thing();
    Post::query();
    Post::missingMethod();
    Post::create(['a' => 1]);
    Post::with('a')->get();
    $post::where('a', 1);
    Post::class::where('a', 1);
    Plain::where();
    $unknown::where('a', 1);
    \App\Post::orderBy('a');
    Post::{'where'}('a');
    Post::$name();
    Post::where(...['a', 1]);
}
-----
<?php

namespace App;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Model;

trait HasThing
{
    public function thing() {}
}

class Post extends Model
{
    use HasThing;

    public static function own() {}

    protected function scopePublished(Builder $q) {}

    public function scopeRecent(Builder $q) {}
}

class Plain
{
    public static function where() {}
}

function run(Post $post, string $name, $unknown)
{
    Post::query()->where('a', 1);
    Post::query()->where('a', 1);
    Post::first();
    Post::find(1);
    Post::query()->published();
    Post::query()->recent();
    Post::query()->Recent();
    Post::own();
    Post::thing();
    Post::query();
    Post::missingMethod();
    Post::create(['a' => 1]);
    Post::with('a')->get();
    $post::query()->where('a', 1);
    Post::class::query()->where('a', 1);
    Plain::where();
    $unknown::where('a', 1);
    \App\Post::query()->orderBy('a');
    Post::{'where'}('a');
    Post::$name();
    Post::query()->where(...['a', 1]);
}
