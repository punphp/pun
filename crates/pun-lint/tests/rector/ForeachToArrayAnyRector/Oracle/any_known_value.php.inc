<?php

function chained()
{
    $items = [1, 2];
    $copy = $items;
    $found = false;
    foreach ($copy as $item) {
        if ($item > 1) {
            $found = true;
            break;
        }
    }
    return $found;
}

function scalar()
{
    $items = 1;
    $found = false;
    foreach ($items as $item) {
        if ($item > 1) {
            $found = true;
            break;
        }
    }
    return $found;
}
-----
<?php

function chained()
{
    $items = [1, 2];
    $copy = $items;
    $found = array_any($copy, fn($item) => $item > 1);
    return $found;
}

function scalar()
{
    $items = 1;
    $found = false;
    foreach ($items as $item) {
        if ($item > 1) {
            $found = true;
            break;
        }
    }
    return $found;
}
