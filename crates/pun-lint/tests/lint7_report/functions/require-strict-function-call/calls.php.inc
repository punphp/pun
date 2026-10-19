<?php

namespace App;

use function array_keys as keys;


function f($a, $b, bool $bool, array $args)
{
    in_array($a, $b);
    in_array($a, $b, true);
    in_array($a, $b, false);
    in_array($a, $b, TRUE);
    in_array($a, $b, \true);
    in_array($a, $b, $bool);
    in_array($a, $b, 1);
    IN_ARRAY($a, $b);
    \in_array($a, $b);
    array_search($a, $b);
    array_search($a, $b, true);
    base64_decode($a);
    base64_decode($a, true);
    base64_decode($a, false);
    array_keys($a);
    array_keys($a, $b);
    array_keys($a, $b, true);
    keys($a, $b);
    in_array(haystack: $b, needle: $a);
    in_array(haystack: $b, needle: $a, strict: true);
    in_array($a, $b, strict: true);
    array_keys($a, strict: true);
    array_keys($a, strict: false);
    array_keys(array: $a, filter_value: $b);
    in_array(...$args);
    in_array($a, ...$args);
    in_array($a, $b, ...$args);
    in_array($a, $b, unknown: true);
    in_array($a, $b, true, 4);
    $x = 'in_array';
    $x($a, $b);
    in_array($a, $b, $bool === true);
    in_array($a, $b, !false);
    in_array($a, $b, (true));
    in_array($a, $b, \PHP_INT_MAX > 0);
}
-----
10: Call to function in_array() requires parameter #3 to be set.
12: Call to function in_array() requires parameter #3 to be true.
15: Call to function in_array() requires parameter #3 to be true.
16: Call to function in_array() requires parameter #3 to be true.
17: Call to function in_array() requires parameter #3 to be set.
18: Call to function in_array() requires parameter #3 to be set.
19: Call to function array_search() requires parameter #3 to be set.
21: Call to function base64_decode() requires parameter #2 to be set.
23: Call to function base64_decode() requires parameter #2 to be true.
25: Call to function array_keys() requires parameter #3 to be set.
27: Call to function array_keys() requires parameter #3 to be set.
28: Call to function in_array() requires parameter #3 to be set.
32: Call to function array_keys() requires parameter #3 to be true.
33: Call to function array_keys() requires parameter #3 to be set.
34: Call to function in_array() requires parameter #3 to be set.
35: Call to function in_array() requires parameter #3 to be set.
36: Call to function in_array() requires parameter #3 to be true.
41: Call to function in_array() requires parameter #3 to be true.
