<?php

use Illuminate\Support\Facades\Config;

Config::collection('offsets.0');

function globalConfigScope(): void
{
    Config::collection('offsets.5e0');
    Config::collection('offsets.+5');
    Config::collection('offsets. 5 ');
    Config::collection('offsets.05');
    Config::collection('offsets.6');
}
