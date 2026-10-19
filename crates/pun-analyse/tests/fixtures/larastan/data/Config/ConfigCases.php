<?php

namespace App\Pun;

use Illuminate\Support\Facades\Config;

class ConfigCases
{
    public function run(string $key): void
    {
        Config::collection('pun.string');
        Config::collection('pun.list');
        Config::collection('pun.int');
        Config::collection('pun.float');
        Config::collection('pun.true');
        Config::collection('pun.null');
        Config::collection('pun.env');
        Config::collection('pun.path');
        Config::collection('pun.concat');
        Config::collection('pun.class');
        Config::collection('pun.interpolated');
        Config::collection('pun.cast');
        Config::collection('pun.nested');
        Config::collection('pun.nested.string');
        Config::collection('pun.nested.array');
        Config::collection('pun.nested.5');
        Config::collection('pun.nested.top');
        Config::collection('pun.nested.missing');
        Config::collection('pun.string.deeper');
        Config::collection('pun.missing');
        Config::collection('pun.7');
        Config::collection('pun');
        Config::collection('nope.key');
        Config::collection('punnested.value');
        Config::collection('nested/punnested.value');
        Config::collection('punlist.a');
        Config::collection('pundoc.a');
        Config::collection('pundoc.b');
        Config::collection('puncall.a');
        Config::collection('pun.string', []);
        Config::collection('pun.list', 'x');
        Config::collection('pun.string', 'x');
        Config::collection('pun.string', fn () => []);
        Config::collection($key);
        Config::collection(rand() ? 'pun.string' : 'pun.list');
        Config::collection(rand() ? 'pun.list' : 'pun.string');
        Config::collection();
        \Illuminate\Support\Facades\Config::collection('pun.int');
        \Config::collection('pun.int');
        Config::Collection('pun.int');
    }
}
