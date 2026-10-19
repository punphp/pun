<?php

function a(array $items)
{
    $found = null;
    foreach ($items as $k => $item) {
        if ($item->id === $k) {
            $found = $item;
            break;
        }
    }
    return $found;
}

function b(array $items)
{
    $key = null;
    foreach ($items as $k => $item) {
        if ($item->id === $k) {
            $key = $k;
            break;
        }
    }
    return $key;
}

function c(array $items)
{
    $found = NULL;
    foreach ($items as $item) {
        if ($item) {
            $found = $item;
            break;
        }
        continue;
    }
    return $found;
}
-----
<?php

function a(array $items)
{
    $found = array_find($items, fn($item, $k) => $item->id === $k);
    return $found;
}

function b(array $items)
{
    $key = null;
    foreach ($items as $k => $item) {
        if ($item->id === $k) {
            $key = $k;
            break;
        }
    }
    return $key;
}

function c(array $items)
{
    $found = NULL;
    foreach ($items as $item) {
        if ($item) {
            $found = $item;
            break;
        }
        continue;
    }
    return $found;
}
