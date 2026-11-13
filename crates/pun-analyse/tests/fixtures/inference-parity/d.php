<?php

declare(strict_types=1);

namespace App;

function mk1(): int
{
    return 1;
}

function t1(): void
{
    $f = null;
    $a = function () use (&$f): void {
        \PHPStan\dumpType($f);
        $f ??= mk1();
        \PHPStan\dumpType($f);
    };
    $g = null;
    $b = function () use (&$g): void {
        \PHPStan\dumpType($g);
        if ($g === null) {
            $g = mk1();
        }
        \PHPStan\dumpType($g);
    };
    $h = null;
    $c = function () use (&$h): void {
        \PHPStan\dumpType($h);
        $h = $h ?? mk1();
        \PHPStan\dumpType($h);
    };
    $i = null;
    $d = function () use (&$i): int {
        \PHPStan\dumpType($i);

        return $i ??= mk1();
    };
    $j = null;
    $e = function () use (&$j): void {
        \PHPStan\dumpType($j);
        $j = 'x';
    };
    $a();
    $b();
    $c();
    $d();
    $e();
}

function t2(): int
{
    static $s = null;
    \PHPStan\dumpType($s);

    return $s ??= mk1();
}
