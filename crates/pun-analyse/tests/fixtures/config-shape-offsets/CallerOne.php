<?php

namespace ConfigShapeOffsets;

use Illuminate\Support\Facades\Config;

class CallerOne
{
    public const KEY = 1;

    public function run(): void
    {
        Config::collection('scoped.0');
    }
}
