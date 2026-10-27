<?php
namespace App\BareHolder;

class D
{
    public function bare(\stdClass $x, bool $c): void
    {
        if ($c) if ($x instanceof \stdClass) {
            echo 1;
            echo 2;
        }
        echo 3;
    }
}
