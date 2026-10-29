<?php

namespace App;

use Illuminate\Contracts\Database\Query\Builder;

class Query implements Builder
{
    public function whereNull($column): static
    {
        return $this;
    }

    public function orderByDesc($column): static
    {
        return $this;
    }
}

/**
 * @mixin Query
 */
class Eloquent implements Builder
{
    public function with($relation): static
    {
        return $this;
    }

    public function orderByDesc($column): static
    {
        return $this;
    }

    public function __call($method, $arguments)
    {
    }
}

/**
 * @mixin Eloquent
 */
class Relation implements Builder
{
    public function __call($method, $arguments)
    {
    }
}

function run(Eloquent $query, Relation $relation)
{
    // a method of the mixin returns the mixin's `static`, here Query, which has no `with()`
    $query->whereNull('a')->with('b')->orderByDesc('created_at');
    $relation->whereNull('a')->with('b')->orderByDesc('created_at');
    $query->with('b')->whereNull('a')->orderByDesc('created_at');
    $relation->with('b')->orderByDesc('created_at');
    $relation->whereNull('a')->orderByDesc('created_at');
}
-----
<?php

namespace App;

use Illuminate\Contracts\Database\Query\Builder;

class Query implements Builder
{
    public function whereNull($column): static
    {
        return $this;
    }

    public function orderByDesc($column): static
    {
        return $this;
    }
}

/**
 * @mixin Query
 */
class Eloquent implements Builder
{
    public function with($relation): static
    {
        return $this;
    }

    public function orderByDesc($column): static
    {
        return $this;
    }

    public function __call($method, $arguments)
    {
    }
}

/**
 * @mixin Eloquent
 */
class Relation implements Builder
{
    public function __call($method, $arguments)
    {
    }
}

function run(Eloquent $query, Relation $relation)
{
    // a method of the mixin returns the mixin's `static`, here Query, which has no `with()`
    $query->whereNull('a')->with('b')->orderByDesc('created_at');
    $relation->whereNull('a')->with('b')->orderByDesc('created_at');
    $query->with('b')->whereNull('a')->latest();
    $relation->with('b')->latest();
    $relation->whereNull('a')->latest();
}
