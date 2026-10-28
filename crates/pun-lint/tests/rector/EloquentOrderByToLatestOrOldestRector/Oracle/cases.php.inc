<?php

namespace App;

use Illuminate\Database\Query\Builder;
use Illuminate\Database\Eloquent\Model;

const DIR = 'asc';

class Cols
{
    public const AT = 'created_at';
    public const DIR = 'asc';
}

enum E: string
{
    case Asc = 'asc';
}

class Post extends Model {}

function run(Builder $query, string $dir, $unknown, $name)
{
    $query
        ->orderBy('created_at');
    $query
        ->orderBy('x_at', 'desc');
    $query->orderBy(
        'x_at',
        'desc'
    );
    $query->orderBy(direction: 'asc', column: 'x_at');
    $query->orderBy(column: 'created_at', direction: 'desc');
    $query->orderBy('created_at', direction: 'desc');
    $query->orderBy('x_at', 'ASC');
    $query->orderBy('x_at', 'foo');
    $query->orderBy('x_at', DIR);
    $query->orderBy('x_at', \DIR);
    $query->orderBy('x_at', Cols::DIR);
    $query->orderBy('x_at', Unknown::DIR);
    $query->orderBy('x_at', E::Asc);
    $query->orderBy('x_at', 'a' . 'sc');
    $query->orderBy('x_at', $dir);
    $query->orderBy('x_at', null);
    $query->orderBy('x_at', 1);
    $query->orderBy(Cols::AT);
    $query->orderBy('created' . '_at');
    $query->orderBy('created_at' /* c */, 'asc');
    $query->orderByDesc('created_at', 'asc');
    $query->orderByDesc('x_at', 'foo');
    $query->orderByDesc('x_at', $dir);
    $query->ORDERBY('x_at');
    $query->orderBy();
    $query->orderBy(...$unknown);
    $query?->orderBy('x_at');
    ($query)
        ->orderBy('created_at');
    (clone $query)
        ->orderBy('created_at');
    (($query))->orderBy('created_at');
    $unknown->orderBy('x_at');
    Post::orderBy('x_at');
    Post::query()->orderBy('x_at');
    $query->orderBy('x_at', 'desc')->orderBy('created_at');
}
-----
<?php

namespace App;

use Illuminate\Database\Query\Builder;
use Illuminate\Database\Eloquent\Model;

const DIR = 'asc';

class Cols
{
    public const AT = 'created_at';
    public const DIR = 'asc';
}

enum E: string
{
    case Asc = 'asc';
}

class Post extends Model {}

function run(Builder $query, string $dir, $unknown, $name)
{
    $query->oldest();
    $query
        ->latest('x_at');
    $query->latest(
        'x_at'
    );
    $query->oldest(column: 'x_at');
    $query->latest();
    $query->latest();
    $query->oldest('x_at');
    $query->latest('x_at');
    $query->latest('x_at');
    $query->latest('x_at');
    $query->oldest('x_at');
    $query->latest('x_at');
    $query->latest('x_at');
    $query->oldest('x_at');
    $query->orderBy('x_at', $dir);
    $query->orderBy('x_at', null);
    $query->orderBy('x_at', 1);
    $query->oldest();
    $query->oldest();
    $query->oldest();
    $query->latest();
    $query->latest('x_at');
    $query->orderByDesc('x_at', $dir);
    $query->oldest('x_at');
    $query->orderBy();
    $query->orderBy(...$unknown);
    $query?->orderBy('x_at');
    $query->oldest();
    (clone $query)->oldest();
    $query->oldest();
    $unknown->orderBy('x_at');
    Post::orderBy('x_at');
    Post::query()->oldest('x_at');
    $query->latest('x_at')->oldest();
}
