<?php

if (rand(0, 1)) {
    $conditional = 31;
}
\PHPStan\dumpType($conditional);

$branch = 41;
if (rand(0, 1)) {
    unset($branch);
}
\PHPStan\dumpType($branch);

function conditionalExtract(array $input, bool $flag): void
{
    extract($input);
    if ($flag) {
        $conditional = 101;
    }
    \PHPStan\dumpType($conditional);
    $known = 103;
    \PHPStan\dumpType($known);
}

$closure = function (array $input, bool $flag): void {
    extract($input);
    if ($flag) {
        $conditional = 107;
    }
    \PHPStan\dumpType($conditional);
};
