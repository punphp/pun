<?php

function arrayPushOracle(array $items, $a, $b, $f)
{
    \array_push($items, $a ? $b : null, $b);
    ARRAY_PUSH($items, $a);
    (array_push($items, $a));
    if ($a) {
        array_push($items, $a, $b);
    }
    if ($a) array_push($items, $a);
    $f($items, $a);
    array_push(...);
    count($items);
    $items;
}
-----
<?php

function arrayPushOracle(array $items, $a, $b, $f)
{
    $items[] = $a ? $b : null;
    $items[] = $b;
    $items[] = $a;
    $items[] = $a;
    if ($a) {
        $items[] = $a;
        $items[] = $b;
    }
    if ($a) $items[] = $a;
    $f($items, $a);
    array_push(...);
    count($items);
    $items;
}
