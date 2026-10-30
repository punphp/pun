<?php

namespace App;

use Illuminate\Support\Collection;

trait Named {}

class Plain {}

/**
 * @param Collection<int, string>|false $maybe
 * @param Collection<int, Named> $traits
 * @param Collection<int, never> $nothing
 */
function run($maybe, $traits, $nothing, Collection $collection)
{
    $maybe->toArray();
    $traits->toArray();
    $nothing->toArray();
    $method = 'all';
    $collection->$method();
    $collection->toArray();
}

class Items extends Collection {}

function items(Items $items)
{
    return $items->toArray();
}
-----
<?php

namespace App;

use Illuminate\Support\Collection;

trait Named {}

class Plain {}

/**
 * @param Collection<int, string>|false $maybe
 * @param Collection<int, Named> $traits
 * @param Collection<int, never> $nothing
 */
function run($maybe, $traits, $nothing, Collection $collection)
{
    $maybe->toArray();
    $traits->all();
    $nothing->toArray();
    $method = 'all';
    $collection->$method();
    $collection->toArray();
}

class Items extends Collection {}

function items(Items $items)
{
    return $items->toArray();
}
