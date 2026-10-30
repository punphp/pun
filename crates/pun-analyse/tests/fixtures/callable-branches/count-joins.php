<?php
namespace CallableCountJoins;

function append(bool $flag): void {
    $arrays = [['first']];
    \PHPStan\dumpType($arrays);
    \PHPStan\dumpType(count($arrays));
    if ($flag) {
        $arrays[] = ['second'];
        \PHPStan\dumpType($arrays);
        \PHPStan\dumpType(count($arrays));
    }
    \PHPStan\dumpType($arrays);
    \PHPStan\dumpType(count($arrays));
    if (count($arrays) > 1) {
        echo 'multiple';
    }
}

function sequential(): void {
    $values = [1];
    $values[] = 2;
    \PHPStan\dumpType($values);
    \PHPStan\dumpType(count($values));
}
