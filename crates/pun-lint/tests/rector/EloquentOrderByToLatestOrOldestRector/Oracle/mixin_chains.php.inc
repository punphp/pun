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

    public function toQuery(): Query
    {
        return new Query();
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

/**
 * @mixin Query|Eloquent
 */
class Either implements Builder
{
    public function __call($method, $arguments)
    {
    }
}

/**
 * @mixin Second
 */
class First implements Builder
{
    public function orderByDesc($column): static
    {
        return $this;
    }

    public function __call($method, $arguments)
    {
    }
}

/**
 * @mixin First
 */
class Second implements Builder
{
    public function orderByDesc($column): static
    {
        return $this;
    }

    public function __call($method, $arguments)
    {
    }
}

function run(Relation $relation, Either $either, First $first, Eloquent|Relation $union, string $name)
{
    // a query builder method returns the query builder again
    $relation->whereNull('a')->whereNull('b')->orderByDesc('created_at');
    // a method missing on the mixin stays missing further down the chain
    $relation->whereNull('a')->with('b')->with('c')->orderByDesc('created_at');
    // a mixin method that returns another class
    $relation->toQuery()->orderByDesc('created_at');
    $relation->$name()->orderByDesc('created_at');
    $either->whereNull('a')->orderByDesc('created_at');
    $first->missing()->orderByDesc('created_at');
    $union->with('b')->orderByDesc('created_at');
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

    public function toQuery(): Query
    {
        return new Query();
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

/**
 * @mixin Query|Eloquent
 */
class Either implements Builder
{
    public function __call($method, $arguments)
    {
    }
}

/**
 * @mixin Second
 */
class First implements Builder
{
    public function orderByDesc($column): static
    {
        return $this;
    }

    public function __call($method, $arguments)
    {
    }
}

/**
 * @mixin First
 */
class Second implements Builder
{
    public function orderByDesc($column): static
    {
        return $this;
    }

    public function __call($method, $arguments)
    {
    }
}

function run(Relation $relation, Either $either, First $first, Eloquent|Relation $union, string $name)
{
    // a query builder method returns the query builder again
    $relation->whereNull('a')->whereNull('b')->latest();
    // a method missing on the mixin stays missing further down the chain
    $relation->whereNull('a')->with('b')->with('c')->orderByDesc('created_at');
    // a mixin method that returns another class
    $relation->toQuery()->latest();
    $relation->$name()->orderByDesc('created_at');
    $either->whereNull('a')->latest();
    $first->missing()->orderByDesc('created_at');
    $union->with('b')->latest();
}
