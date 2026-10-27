<?php
namespace RepeatByteComparisons;
use function PHPStan\dumpType;

/**
 * @param lowercase-string $lower
 * @param uppercase-string $upper
 */
function comparisons(string $lower, string $upper): void
{
    switch ($lower) {
        case "\xff": break;
        case 'É': break;
        case 'A': break;
    }
    switch ($upper) {
        case "\xff": break;
        case 'é': break;
        case 'a': break;
    }
    dumpType(987);
}
