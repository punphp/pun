<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

class Post extends Model
{
    public static function recent()
    {
        return self::where('a', 1);
    }

    public static function latestOnes()
    {
        return static::where('a', 1);
    }
}

class Article extends Post
{
    public static function fromParent()
    {
        return parent::where('a', 1);
    }
}

function run()
{
    $class = 'App\Post';
    $class::where('a', 1);
}
-----
<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

class Post extends Model
{
    public static function recent()
    {
        return self::query()->where('a', 1);
    }

    public static function latestOnes()
    {
        return static::query()->where('a', 1);
    }
}

class Article extends Post
{
    public static function fromParent()
    {
        return parent::query()->where('a', 1);
    }
}

function run()
{
    $class = 'App\Post';
    $class::query()->where('a', 1);
}
