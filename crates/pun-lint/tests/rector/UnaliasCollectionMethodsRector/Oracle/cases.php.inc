<?php

namespace App;

use Illuminate\Support\Collection;

class Cases
{
    public function run(Collection $items, ?Collection $maybe, $unknown)
    {
        $items->some(fn ($x) => $x);
        $items->AVERAGE('price');
        $maybe->unlessEmpty(fn () => null);
        $items->unlessNotEmpty(fn () => null)->average();
        $items?->some(fn ($x) => $x);
        $unknown->average();
        $items->avg();
        $items->{'some'}();
    }
}
-----
<?php

namespace App;

use Illuminate\Support\Collection;

class Cases
{
    public function run(Collection $items, ?Collection $maybe, $unknown)
    {
        $items->contains(fn ($x) => $x);
        $items->avg('price');
        $maybe->whenNotEmpty(fn () => null);
        $items->whenEmpty(fn () => null)->average();
        $items?->some(fn ($x) => $x);
        $unknown->average();
        $items->avg();
        $items->{'some'}();
    }
}
