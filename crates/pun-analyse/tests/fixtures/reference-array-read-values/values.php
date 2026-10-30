<?php

namespace ReferenceArrayReadValues;

function referenceInputs(): void
{
    $known = 1;
    $items = ['known' => &$known, 'unknown' => &$unknown]; // observe-array
    $legacy = array(&$legacyUnknown); // observe-array
    \PHPStan\dumpType($items, $legacy);
}

function referenceReadControls(int $nativeControl, mixed $genuineMixed): array
{
    $nativeControl; // observe-native
    $known = 23;
    $items = ['known' => &$known, 'unknown' => &$missing]; // observe-array
    \PHPStan\dumpType($items);
    \PHPStan\dumpType($known); // observe
    \PHPStan\dumpType($missing); // observe
    $legacy = array(&$legacyMissing); // observe-array
    \PHPStan\dumpType($legacy);
    $ordinary = [$plainMissing]; // observe-array
    \PHPStan\dumpType($ordinary);
    $mixed = [&$genuineMixed]; // observe-array
    \PHPStan\dumpType($mixed);
    $alias =& $known;
    \PHPStan\dumpType($alias); // observe
    return $items;
}

function referenceAfterUnset(): array
{
    $absent = 31;
    unset($absent);
    $items = [&$absent]; // observe-array
    \PHPStan\dumpType($items);
    \PHPStan\dumpType($absent); // observe
    return $items;
}

function referenceOffsets(): array
{
    $source = ['known' => 41];
    $known = [&$source['known']]; // observe-array
    \PHPStan\dumpType($known);
    return $known;
}

function referenceEscape(): void
{
    $known = 47;
    $escaped = [&$known]; // observe-array
    $known = 53;
}
