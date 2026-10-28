<?php
namespace CallableCountCertainty;

function literals(): void {
    if (count([1, 2]) === 2) {
        echo 'two';
    }
    if (sizeof([1]) !== 1) {
        echo 'not one';
    }
}

/** @param array{int, int} $array */
function shape(array $array): void {
    if (count($array) === 2) {
        echo 'two';
    }
}

/**
 * @param non-empty-list<list<int>> $nested
 * @param list<int> $flat
 */
function recursiveViews(array $nested, array $flat, \Countable $countable, int $mode): void {
    \PHPStan\dumpType(count($nested, COUNT_RECURSIVE));
    \PHPStan\dumpType(sizeof($flat, $mode));
    \PHPStan\dumpType(count($countable));
    \PHPStan\dumpType(count([]));
    \PHPStan\dumpType(count($nested, COUNT_NORMAL));
}
