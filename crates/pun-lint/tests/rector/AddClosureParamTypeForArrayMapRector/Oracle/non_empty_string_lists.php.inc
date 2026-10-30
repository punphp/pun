<?php

namespace Probe;

/**
 * @param  non-empty-list<non-empty-string>  $a
 * @param  non-empty-list<string>  $b
 */
function run(array $a, array $b)
{
    array_map(fn ($x) => $x, $a);
    array_map(fn ($x) => $x, $b);
}
