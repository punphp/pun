<?php

function shapes(array $animals)
{
    echo 1;
    foreach ($animals as $animal) {
        if (str_starts_with($animal, 'c')) {
            $found = $animal;
            break;
        }
    }

    foo();
    foreach ($animals as $animal) {
        if (str_starts_with($animal, 'c')) {
            $found = $animal;
            break;
        }
    }

    $found = null;
    foreach ($animals as $animal) {
        if (str_starts_with($animal, 'c')) {
            $found = $animal;
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
    foreach ($animals as $animal) {
        if (str_starts_with($animal, 'c')) {
            $found = $animal;
            echo 1;
            break;
        }
    }

    $found = null;
    foreach ($animals as $animal) {
        if (str_starts_with($animal, 'c')) {
            break;
        }
    }

    return $found;
}
