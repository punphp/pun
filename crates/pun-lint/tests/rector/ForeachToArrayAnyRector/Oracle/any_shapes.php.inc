<?php

function shapes(array $animals)
{
    echo 1;
    foreach ($animals as $animal) {
        if (str_starts_with($animal, 'c')) {
            $found = true;
            break;
        }
    }

    foo();
    foreach ($animals as $animal) {
        if (str_starts_with($animal, 'c')) {
            $found = true;
            break;
        }
    }

    $found = false;
    foreach ($animals as $animal) {
        if (str_starts_with($animal, 'c')) {
            $found = true;
            break;
            // trailing
        }
    }

    $found = false;
    foreach ($animals as [$a, $b]) {
        if (str_starts_with($a, 'c')) {
            $found = true;
            break;
        }
    }

    $found = false;
    foreach ($animals as $animal) {
        if (str_starts_with($animal, 'c')) {
            $found = true;
            echo 1;
            break;
        }
    }

    $found = false;
    foreach ($animals as $animal) {
        if (str_starts_with($animal, 'c')) {
            break;
        }
    }

    return $found;
}
