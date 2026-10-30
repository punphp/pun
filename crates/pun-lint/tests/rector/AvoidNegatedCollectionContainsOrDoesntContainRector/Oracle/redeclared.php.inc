<?php

namespace App;

use Illuminate\Support\Collection;

function reset(Collection $items)
{
    return $items;
}

function reset(Collection $items)
{
    $items = null;

    return ! $items->contains(1);
}
