<?php

namespace App\Pun;

use Illuminate\Support\Facades\Config;

class ConfigExtraCases
{
    public function run(): void
    {
        Config::collection('punlegacy.a');
        Config::collection('punlegacy.b');
        Config::collection('punlegacy.b.c');
        Config::collection('punlegacy.1.5');
        Config::collection('punlegacy.var');
        Config::collection('punlegacy.dynamic');
        Config::collection('punlegacy.unknown');
        Config::collection('punlegacy.object');
        Config::collection('punlegacy.enum');
        Config::collection('punlegacy.closure');
        Config::collection('punlegacy.callable');
        Config::collection('punlegacy.mixed');
        Config::collection('punlegacy.envarray');
        Config::collection('punlegacy.envnull');
        Config::collection('punlegacy.intcast');
        Config::collection('punlegacy.not');
        Config::collection('punlegacy.after');
        Config::collection('pundoc2.a');
        Config::collection('pundoc2.b');
        Config::collection('pundoc2.c');
        Config::collection('pundoc2.d');
        Config::collection('pundoc2.d.x');
        Config::collection('pundoc2.e.0.x');
        Config::collection('pundoc2.e.0.y');
        Config::collection('pundoc2.f.0');
        Config::collection('pundoc2.g.x');
        Config::collection('pundoc2.h');
        Config::collection('pundoc2.i');
        Config::collection('pundoc2.j');
        Config::collection('pundoc2.missing');
        Config::collection('punbroken.a');
        Config::collection('punspread.a');
        Config::collection('punnoreturn.a');
        Config::collection('pundoc2.a', fn (): string => 'x');
        Config::collection('nofile.key', 'x');
    }
}
