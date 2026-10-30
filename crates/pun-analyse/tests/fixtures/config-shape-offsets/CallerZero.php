<?php

namespace ConfigShapeOffsets;

use Illuminate\Support\Facades\Config;

class CallerZero
{
    public const KEY = 0;

    public function run(): void
    {
        Config::collection('scoped.0');
    }
}
