<?php

function arrayPushLowPrecedence(array $items, $a, $b)
{
    array_push($items, $a and $b);
}
-----
<?php

function arrayPushLowPrecedence(array $items, $a, $b)
{
    $items[] = ($a and $b);
}
