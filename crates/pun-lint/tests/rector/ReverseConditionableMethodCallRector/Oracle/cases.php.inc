<?php

namespace App;

use Illuminate\Support\Traits\Conditionable;

class Query
{
    use Conditionable;
}

class Child extends Query
{
}

function run(Query $query, Child $child, ?Query $maybe, $unknown, bool $a, bool $b)
{
    $query->When(!$a, fn () => null);
    $query->UNLESS(!$a, fn () => null);
    $child->when(value: !$a, callback: fn () => null);
    $maybe->unless((!$a), fn () => null);
    $query->when(!($a && $b), fn () => null);
    $query->when(! $a)->unless(!$b);
    $query->when($a, fn () => null);
    $query?->when(!$a, fn () => null);
    $unknown->when(!$a, fn () => null);
    $query->when();
}
-----
<?php

namespace App;

use Illuminate\Support\Traits\Conditionable;

class Query
{
    use Conditionable;
}

class Child extends Query
{
}

function run(Query $query, Child $child, ?Query $maybe, $unknown, bool $a, bool $b)
{
    $query->when($a, fn () => null);
    $query->when($a, fn () => null);
    $child->unless($a, callback: fn () => null);
    $maybe->unless((!$a), fn () => null);
    $query->unless($a && $b, fn () => null);
    $query->unless($a)->unless(!$b);
    $query->when($a, fn () => null);
    $query?->when(!$a, fn () => null);
    $unknown->when(!$a, fn () => null);
    $query->when();
}
