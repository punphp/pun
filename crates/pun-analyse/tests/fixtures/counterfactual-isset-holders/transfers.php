<?php

$target = 17;
$alias =& $target;
if (!isset($alias)) {
    \PHPStan\dumpType($alias); // observe
    \PHPStan\dumpType($target); // observe
    $alias = 23;
    \PHPStan\dumpType($alias); // observe
}
\PHPStan\dumpType($alias); // observe
\PHPStan\dumpType($target); // observe

$array = ['key' => null];
if (!isset($array['key'])) {
    \PHPStan\dumpType($array);
}

function unrelatedCounterfactual(int $unrelated): void
{
    $unrelated; // observe-native
    $known = 31;
    $known; // observe-native
    if (!isset($known)) {
        \PHPStan\dumpType($unrelated); // observe
    }
    \PHPStan\dumpType($unrelated); // observe
    echo 'unrelated control';
}

function extractedReferenceCounterfactual(array $input): void
{
    extract($input);
    $target = 37;
    $alias =& $target;
    if (!isset($target)) {
        \PHPStan\dumpType($target); // observe
        \PHPStan\dumpType($alias); // observe
        $alias = 41;
        \PHPStan\dumpType($target); // observe
        \PHPStan\dumpType($alias); // observe
    }
    \PHPStan\dumpType($target); // observe
    \PHPStan\dumpType($alias); // observe
}
