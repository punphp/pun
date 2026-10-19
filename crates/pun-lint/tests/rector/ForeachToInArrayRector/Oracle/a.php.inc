<?php

function a(array $items)
{
    $x = 1;

    // lead
    foreach ($items as $item) {
        if ($item === 'a') {
            return true;
        }
    }

    // before return
    return false;
}

function b(array $items)
{
    $x = 1;
    foreach ($items as $item) {
        if ($item == 'a') {
            return false;
        }
    }
    return true;
}

function c(array $items)
{
    foreach ($items as $item) {
        if ($item === $a ?: $b) {
            return true;
        }
    }

    return false;
}
-----
<?php

function a(array $items)
{
    $x = 1;
    return in_array('a', $items, true);
}

function b(array $items)
{
    $x = 1;
    return !in_array('a', $items);
}

function c(array $items)
{
    foreach ($items as $item) {
        if ($item === $a ?: $b) {
            return true;
        }
    }

    return false;
}
