<?php

if (rand(0, 1)) {
    $conditional = 31;
}
$conditional; // observe

$branch = 41;
if (rand(0, 1)) {
    unset($branch);
}
$branch; // observe

function valuesAfterExtract(array $input, bool $flag): void
{
    extract($input);
    if ($flag) {
        $conditional = 101;
    }
    $conditional; // observe
    $known = 103;
    $known; // observe
}

function absentRefinement(bool $flag): void
{
    $known = 17;
    $known; // observe-native
    unset($absent);
    if ($flag) {
        $maybe = 19;
    }
    $maybe; // observe
    if (!isset($known)) {
        $known; // observe
    }
}
