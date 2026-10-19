<?php

function a(array $items)
{
    $ok = true;

    foreach ($items as $item) {
        if (! $item) {
            $ok = false;
            break;
        }
    }

    return $ok;
}

function b(array $items)
{
    echo 1;
    $ok = true;
    foreach ($items as $item) {
        if (! $item) {
            $ok = false;
            break;
        }
    }

    echo 2;

    return $ok;
}

function c(array $items)
{
    echo 1;

    // before foreach
    foreach ($items as $item) {
        if (! $item) {
            return false;
        }
    }

    // before return
    return true;
}

function d(array $items)
{
    foreach ($items as $item) {
        if (! $item) {
            return false;
        }
    }
    return true;
    // after
}
-----
<?php

function a(array $items)
{
    $ok = array_all($items, fn($item) => $item);
    return $ok;
}

function b(array $items)
{
    echo 1;
    $ok = array_all($items, fn($item) => $item);

    echo 2;

    return $ok;
}

function c(array $items)
{
    echo 1;
    return array_all($items, fn($item) => $item);
}

function d(array $items)
{
    return array_all($items, fn($item) => $item);
    // after
}
