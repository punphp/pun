<?php

namespace App;

use Illuminate\Support\Collection;

class Cases
{
    public function run(Collection $items, $unknown, bool $flag)
    {
        $items->filter(fn ($x): int => !$x);
        $items->reject(fn &($x) => !($x instanceof Collection));
        $items->filter(callback: fn ($x) => !$flag);
        $items->filter((fn ($x) => !$x));
        $items->filter(fn ($x) => (!$x));
        $items->filter(fn ($x) => !$x + 1);
        $items->filter(function ($x) { return !$x; });
        $items->filter(fn ($x) => !$x, 1);
        $unknown->filter(fn ($x) => !$x);
        $items?->filter(fn ($x) => !$x);
        $items->reject(fn (int $x) => !($x . 'a'));
    }
}
-----
<?php

namespace App;

use Illuminate\Support\Collection;

class Cases
{
    public function run(Collection $items, $unknown, bool $flag)
    {
        $items->reject(fn ($x): bool => (bool) $x);
        $items->filter(fn &($x): bool => $x instanceof Collection);
        $items->reject(callback: fn ($x): bool => $flag);
        $items->reject((fn ($x): bool => (bool) $x));
        $items->reject(fn ($x): bool => ((bool) $x));
        $items->filter(fn ($x) => !$x + 1);
        $items->filter(function ($x) { return !$x; });
        $items->filter(fn ($x) => !$x, 1);
        $unknown->filter(fn ($x) => !$x);
        $items?->filter(fn ($x) => !$x);
        $items->filter(fn (int $x): bool => (bool) ($x . 'a'));
    }
}
