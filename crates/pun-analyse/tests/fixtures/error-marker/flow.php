<?php

function markerFlow(bool $condition, mixed $mixed): void
{
    $error = error_value();
    \PHPStan\dumpType($error);
    $copy = $error;
    \PHPStan\dumpType($copy);
    \PHPStan\dumpType($copy['unknown']);
    $union = $condition ? $error : 13;
    \PHPStan\dumpType($union);
    $reverse = $condition ? 13 : $error;
    \PHPStan\dumpType($reverse);
    if ($copy !== null) {
        \PHPStan\dumpType($copy);
    }
    if ($error) {
        \PHPStan\dumpType($error);
    }
    if (empty($copy)) {
        \PHPStan\dumpType($copy);
    }
    $copy = 3;
    \PHPStan\dumpType($copy);
    \PHPStan\dumpType($mixed);
}
