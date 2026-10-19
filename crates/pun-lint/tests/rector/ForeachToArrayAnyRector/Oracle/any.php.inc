<?php

function a(array $items)
{
    $found = false;
    foreach ($items as $k => $item) {
        if ($item === $k) {
            $found = true;
            break;
        }
    }
    return $found;
}

function b(array $items)
{
    foreach ($items as $item) {
        if ($item instanceof Foo) {
            return true;
        }
    }

    return false;
}

function c(array $items)
{
    $found = false;
    foreach ($items as $item) {
        if ($item) {
            $found = true;
            break;
        }
    }
    echo $item;
}

function d(array $items)
{
    $found = false;
    foreach ((array) $items as $item) if ($item) { $found = true; break; }
}

function e(array $items)
{
    $found = false;
    foreach ($items as $item) {
        if ($item) {
            $other = true;
            break;
        }
    }
}
-----
<?php

function a(array $items)
{
    $found = array_any($items, fn($item, $k) => $item === $k);
    return $found;
}

function b(array $items)
{
    return array_any($items, fn($item) => $item instanceof Foo);
}

function c(array $items)
{
    $found = false;
    foreach ($items as $item) {
        if ($item) {
            $found = true;
            break;
        }
    }
    echo $item;
}

function d(array $items)
{
    $found = array_any((array) $items, fn($item) => $item);
}

function e(array $items)
{
    $found = false;
    foreach ($items as $item) {
        if ($item) {
            $other = true;
            break;
        }
    }
}
