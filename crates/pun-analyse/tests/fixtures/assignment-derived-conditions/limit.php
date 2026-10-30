<?php declare(strict_types=1);

namespace AssignmentDerivedConditions;

function bounded(bool $f0, bool $f1, bool $f2, bool $f3, bool $f4, bool $f5, bool $f6, bool $f7, bool $f8): bool
{
    if ($f0) { $t0 = 'first'; $v0 = 'left'; } else { $t0 = 'second'; $v0 = 'right'; }
    if ($f1) { $t1 = 'first'; $v1 = 'left'; } else { $t1 = 'second'; $v1 = 'right'; }
    if ($f2) { $t2 = 'first'; $v2 = 'left'; } else { $t2 = 'second'; $v2 = 'right'; }
    if ($f3) { $t3 = 'first'; $v3 = 'left'; } else { $t3 = 'second'; $v3 = 'right'; }
    if ($f4) { $t4 = 'first'; $v4 = 'left'; } else { $t4 = 'second'; $v4 = 'right'; }
    if ($f5) { $t5 = 'first'; $v5 = 'left'; } else { $t5 = 'second'; $v5 = 'right'; }
    if ($f6) { $t6 = 'first'; $v6 = 'left'; } else { $t6 = 'second'; $v6 = 'right'; }
    if ($f7) { $t7 = 'first'; $v7 = 'left'; } else { $t7 = 'second'; $v7 = 'right'; }
    if ($f8) { $t8 = 'first'; $v8 = 'left'; } else { $t8 = 'second'; $v8 = 'right'; }
    $copy = [$v0, $v1, $v2, $v3, $v4, $v5, $v6, $v7, $v8];
    if ($t0 === 'first') { \PHPStan\dumpType($copy[0]); }
    if ($t3 === 'first') { \PHPStan\dumpType($copy[3]); }
    if ($t4 === 'first') { \PHPStan\dumpType($copy[4]); }
    if ($t8 === 'first') { \PHPStan\dumpType($copy[8]); }
    return true;
}
