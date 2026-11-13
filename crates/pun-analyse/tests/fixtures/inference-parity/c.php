<?php

declare(strict_types=1);

namespace App;

final class Lazy
{
    public function make(): int
    {
        return 1;
    }

    /** @return \Closure(): int */
    public function a(): \Closure
    {
        $f = null;

        return function () use (&$f): int {
            return $f ??= $this->make();
        };
    }

    /** @return \Closure(): int */
    public function b(): \Closure
    {
        $f = null;
        $g = function () use (&$f): int {
            \PHPStan\dumpType($f);

            return $f ??= $this->make();
        };
        \PHPStan\dumpType($f);

        return $g;
    }

    public function c(): void
    {
        $n = 0;
        $inc = function () use (&$n): void {
            $n++;
        };
        $inc();
        $s = 'a';
        $h = fn (): string => $s;
        $k = function () use (&$s): string {
            \PHPStan\dumpType($s);

            return $s;
        };
        $s = 'b';
        \PHPStan\dumpType($k(), $h());
    }
}
