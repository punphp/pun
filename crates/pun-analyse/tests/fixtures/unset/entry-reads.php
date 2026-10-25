<?php

namespace UnsetEntryReads;

function repeatedScalar(): void
{
    $array = ['z' => ['leaf' => 7, 'keep' => 3], 'a' => 2];
    unset($array['z']['leaf']['bad'], $array['z']['leaf']['again']);
    \PHPStan\dumpType($array);
}

/** @param array{z: mixed, a: int} $array */
function narrowerIndexAssignment(array $array): void
{
    unset($array[$array = ['a' => 1, 'z' => 2]]);
    \PHPStan\dumpType($array);
}

function incompatibleIndexAssignment(): void
{
    $array = 7;
    unset($array[$array = ['new' => 4]]);
    \PHPStan\dumpType($array);
}

function newlyCreatedReceiver(): void
{
    unset($array[$array = ['new' => 4]]);
    \PHPStan\dumpType($array);
}

function realNullReceiver(): void
{
    $array = null;
    unset($array[$array = ['new' => 4]]);
    \PHPStan\dumpType($array);
}

function indexSideEffects(): void
{
    $index = 0;
    $array = [['leaf' => 7], ['leaf' => 8], 'safe' => 3];
    unset($array[$index++]['leaf']['bad'], $array[$index++]['leaf']['again']);
    \PHPStan\dumpType($index);
    \PHPStan\dumpType($array);
}
