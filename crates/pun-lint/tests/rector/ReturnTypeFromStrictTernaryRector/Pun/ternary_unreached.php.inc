<?php

namespace App;

final class Unreached
{
    public function afterExit(bool $x)
    {
        exit(1);

        return $x ? 1 : 2;
    }
}
