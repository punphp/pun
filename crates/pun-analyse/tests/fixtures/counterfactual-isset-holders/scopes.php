<?php

$refined = 53;
if (!isset($refined)) {
    \PHPStan\dumpType($refined); // observe
    \PHPStan\dumpType($refined); // observe
}
\PHPStan\dumpType($refined); // observe

$nullable = rand(0, 1) ? null : 59;
if (!isset($nullable)) {
    \PHPStan\dumpType($nullable); // observe
}

function extractedCounterfactual(array $input): void
{
    extract($input);
    $known = 103;
    if (!isset($known)) {
        \PHPStan\dumpType($known); // observe
    }
    \PHPStan\dumpType($known); // observe
}

function localCounterfactual(): void
{
    $known = 71;
    if (!isset($known)) {
        \PHPStan\dumpType($known); // observe
    }
    \PHPStan\dumpType($known); // observe
    echo 'local control';
}

$localClosure = function (): void {
    $known = 61;
    if (!isset($known)) {
        \PHPStan\dumpType($known); // observe
    }
    \PHPStan\dumpType($known); // observe
};

$extractedClosure = function (array $input): void {
    extract($input);
    $known = 109;
    if (!isset($known)) {
        \PHPStan\dumpType($known); // observe
    }
    \PHPStan\dumpType($known); // observe
    $nested = function (): void {
        $inner = 113;
        if (!isset($inner)) {
            \PHPStan\dumpType($inner); // observe
        }
    };
};

function genuinelyAbsentCounterfactual(bool $flag): void
{
    if ($flag) {
        $tag = 1;
        $onlyWhenTrue = 127;
    } else {
        $tag = 2;
    }
    if ($tag === 2) {
        \PHPStan\dumpType($onlyWhenTrue); // observe-native
        if (!isset($onlyWhenTrue)) {
            \PHPStan\dumpType($onlyWhenTrue); // observe
        }
    }
    echo 'absent control';
}

function absentLocalCounterfactual(): void
{
    if (!isset($missing)) {
        \PHPStan\dumpType($missing); // observe
    }
    if (isset($missing)) {
        if (!isset($missing)) {
            \PHPStan\dumpType($missing); // observe
        }
    }
    echo 'nested absent control';
}
