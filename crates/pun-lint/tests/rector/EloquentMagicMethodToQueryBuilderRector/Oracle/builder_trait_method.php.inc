<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

class Post extends Model {}

class Repo
{
    public function run(): void
    {
        Post::each(fn ($post) => 1);
        Post::count();
        Post::query()->each(fn ($post) => 1);
        Post::notABuilderMethod();
    }
}
-----
<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

class Post extends Model {}

class Repo
{
    public function run(): void
    {
        Post::query()->each(fn ($post) => 1);
        Post::count();
        Post::query()->each(fn ($post) => 1);
        Post::notABuilderMethod();
    }
}
