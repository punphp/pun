<?php

namespace App;

use Illuminate\Support\Env as E;

env('A') ?? null;
data_get($a, 'k') ?? 'd';
data_get($a) ?? 'd';
E::get('A') ?? 'x';
$obj->config('a') ?? 1;
Foo::env('a') ?? 1;
(config('a')) ?? 1;
config() ?? 1;
config('a') ?? $b ?? $c;
$a ?? config('x');
\config('a') ?? [1, 2];
$obj->get('a') ?? 1;
Other::get('a') ?? 1;
$x = null;
config('a') ?? $x;
config('a', 'b') ?? 'c';
-----
<?php

namespace App;

use Illuminate\Support\Env as E;

env('A');
data_get($a, 'k', 'd');
data_get($a) ?? 'd';
E::get('A', 'x');
$obj->config('a', 1);
Foo::env('a', 1);
config('a', 1);
config() ?? 1;
config('a', $b ?? $c);
$a ?? config('x');
\config('a', [1, 2]);
$obj->get('a') ?? 1;
Other::get('a') ?? 1;
$x = null;
config('a');
config('a', 'b') ?? 'c';
