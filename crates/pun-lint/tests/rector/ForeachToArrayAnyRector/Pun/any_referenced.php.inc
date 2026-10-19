<?php

function referenced()
{
    $items = [1, 2];
    $alias = &$items;
    $found = false;
    foreach ($items as $item) {
        if ($item > 1) {
            $found = true;
            break;
        }
    }
    return $found;
}
