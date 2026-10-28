<?php

namespace App;

final class Unreachable
{
    public function afterExit(string $s)
    {
        exit(1);

        return strlen($s);
    }
}
