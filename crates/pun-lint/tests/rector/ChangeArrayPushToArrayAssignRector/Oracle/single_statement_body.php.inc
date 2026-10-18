<?php

function arrayPushSingleStatementBody(array $items, $a, $b)
{
    if ($a) array_push($items, $a, $b);
}
-----
<?php

function arrayPushSingleStatementBody(array $items, $a, $b)
{
    if ($a) {
        $items[] = $a;
        $items[] = $b;
    }
}
