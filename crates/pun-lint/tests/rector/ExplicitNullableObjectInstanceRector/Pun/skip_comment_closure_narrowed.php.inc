<?php

namespace App\Probe;

final class Circle {}

final class Kept
{
    public function run(?Circle $q, ?\Closure $e)
    {
        if (! /* note */ $q) { echo 1; }
        if ($q /* why */ === null) { echo 1; }
        if ($e) { echo 1; }
        if ($q instanceof Circle && $q) { echo 1; }
    }
}
