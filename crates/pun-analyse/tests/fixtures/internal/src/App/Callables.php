<?php

namespace App;

use Vendor\Pkg\Lib;
use Vendor\Pkg\Secret;

class Callables extends Lib
{
    public function make(Lib $lib, Secret $secret): void
    {
        $a = $lib->hidden(...);
        $b = $lib->open(...);
        $c = Lib::hiddenStatic(...);
        $d = $secret->run(...);
        $e = Secret::make(...);
        $f = self::hiddenStatic(...);
        $g = parent::hiddenStatic(...);
        $h = $lib::hiddenStatic(...);
    }
}
