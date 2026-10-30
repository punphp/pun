<?php

namespace App\Unreachable;

use stdClass;

class C
{
    public function unreachable(stdClass $var)
    {
        return;
        if ($var instanceof stdClass) {
            echo 4;
        }
    }
}
