<?php

namespace ConfigShapeOffsets;

use Illuminate\Support\Facades\Config;

function outside(): void
{
    Config::collection('scoped.0');
}
