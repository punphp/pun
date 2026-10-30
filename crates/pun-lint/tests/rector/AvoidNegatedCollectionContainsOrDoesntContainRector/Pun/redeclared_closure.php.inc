<?php

namespace App;

use Illuminate\Support\Collection;

function check(Collection $items)
{
    return $items;
}

function check(Collection $items)
{
    // Mago does not analyse a second declaration of a function, and Pun reads the parameter's hint only where the
    // function itself uses it; Rector analyses it as it is and changes `! $items->contains(1)` in the arrow function.
    return fn () => ! $items->contains(1);
}
