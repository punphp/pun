<?php

function shapes(array $animals)
{
    echo 1;
    foreach ($animals as $k => $animal) {
        if (str_starts_with($animal, 'c')) {
            $found = $k;
            break;
        }
    }

    foo();
    foreach ($animals as $k => $animal) {
        if (str_starts_with($animal, 'c')) {
            $found = $k;
            break;
        }
    }

    $found = null;
    foreach ($animals as $k => $animal) {
        if (str_starts_with($animal, 'c')) {
            $found = $k;
            break;
            // trailing
        }
    }

    $found = null;
    foreach ($animals as [$a, $b]) {
        if (str_starts_with($a, 'c')) {
            $found = $a;
            break;
        }
    }

    $found = null;
    foreach ($animals as $k => $animal) {
        if (str_starts_with($animal, 'c')) {
            $found = $k;
            echo 1;
            break;
        }
    }

    $found = null;
    foreach ($animals as $k => $animal) {
        if (str_starts_with($animal, 'c')) {
            break;
        }
    }

    return $found;
}
