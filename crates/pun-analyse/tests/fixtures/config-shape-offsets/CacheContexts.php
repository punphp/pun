<?php

namespace ConfigShapeOffsets;

use Illuminate\Support\Facades\Config;

class FirstLookup
{
    public const KEY = 0;

    public function run(): void
    {
        Config::collection('scoped.0');
    }
}

class LaterLookup
{
    public const KEY = 1;

    public function run(): void
    {
        Config::collection('scoped.0');
    }
}
