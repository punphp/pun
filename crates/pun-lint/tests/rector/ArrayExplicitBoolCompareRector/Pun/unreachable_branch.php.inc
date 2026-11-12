<?php

function arrayExplicitBoolCompareUnreachable(array $items)
{
    if ($items) {
        return 1;
    } elseif (!$items) {
        return 2;
    } else if ($items) {
        return 3;
    }
}
-----
<?php

function arrayExplicitBoolCompareUnreachable(array $items)
{
    if ($items !== []) {
        return 1;
    } elseif ($items === []) {
        return 2;
    } else if ($items !== []) {
        return 3;
    }
}
