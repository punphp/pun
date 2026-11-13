<?php

declare(strict_types=1);

namespace App;

final class E
{
    public function a(string $title): void
    {
        if (preg_match('/^([a-z]+)(!)?: (\S.*)$/i', $title, $m2) === 1 && $title !== 'x') {
            \PHPStan\dumpType($m2);
        }
    }
}
