<?php

function more(array $animals)
{
    $found = null;
    foreach ($animals as $animal) {
        echo $animal;
    }

    $found ??= null;
    foreach ($animals as $animal) {
        if (str_starts_with($animal, 'c')) {
            $found = $animal;
            break;
        }
    }

    $found = null;
    foreach ($animals as $animal) {
        if (str_starts_with($animal, 'c')) {
            echo $animal;
            break;
        }
    }

    return $found;
}
