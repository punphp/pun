<?php

$value = 17;
unset($value);
\PHPStan\dumpType($value); // observe
\PHPStan\dumpType($value); // observe
unset($value);
\PHPStan\dumpType($value); // observe
$value = 23;
\PHPStan\dumpType($value); // observe

$closure = function (): void {
    $local = 61;
    unset($local);
    \PHPStan\dumpType($local); // observe
};

function extractedLater(array $input): void
{
    $local = 113;
    unset($local);
    \PHPStan\dumpType($local); // observe
    extract($input);
    \PHPStan\dumpType($local); // observe
}

function localScope(bool $flag): void
{
    $local = 71;
    unset($local);
    \PHPStan\dumpType($local); // observe
    if ($flag) {
        $maybe = 83;
    }
    \PHPStan\dumpType($maybe); // observe
}

function extracted(array $input): void
{
    extract($input);
    $local = 97;
    unset($local);
    \PHPStan\dumpType($local); // observe
    $known = 103;
    \PHPStan\dumpType($known); // observe
    $inner = function (): void {
        $isolated = 107;
        unset($isolated);
        \PHPStan\dumpType($isolated); // observe
    };
}

$extractedClosure = function (array $input): void {
    extract($input);
    $local = 109;
    unset($local);
    \PHPStan\dumpType($local); // observe
};
