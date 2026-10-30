<?php

namespace ConfigShapeOffsets;

use Illuminate\Support\Facades\Config;

class CacheFirstMissing
{
    public const KEY = 1;

    public function run(): void
    {
        Config::collection('scoped.0');
        Config::collection('scoped.1');
        Config::collection('literal');
        Config::collection('literal');
        Config::collection('literal.named');
    }
}

class CacheLaterScalar
{
    public const KEY = 0;

    public function run(): void
    {
        Config::collection('scoped.0');
        Config::collection('scoped.1');
        Config::collection('literal.named', []);
    }
}
