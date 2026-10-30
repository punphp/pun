<?php

namespace App;

use Illuminate\Support\Collection;
use Illuminate\Support\Enumerable;

class Cases
{
    private Collection $items;

    public function run(?Collection $maybe, Enumerable $enumerable, array $array, $unknown)
    {
        if (!$this->items->contains('a')) {
        }
        $a = !($maybe->CONTAINS('b'));
        $b = !$enumerable->doesntContain(fn ($x) => $x > 1);
        $c = ! $this->items?->contains('a');
        $d = !$unknown->contains('a');
        $e = !$this->items->has('a');
        $f = !(new Collection([1]))->contains(1);
        $g = !collect([1])->contains(1);
    }
}
-----
<?php

namespace App;

use Illuminate\Support\Collection;
use Illuminate\Support\Enumerable;

class Cases
{
    private Collection $items;

    public function run(?Collection $maybe, Enumerable $enumerable, array $array, $unknown)
    {
        if ($this->items->doesntContain('a')) {
        }
        $a = $maybe->doesntContain('b');
        $b = $enumerable->contains(fn ($x) => $x > 1);
        $c = ! $this->items?->contains('a');
        $d = !$unknown->contains('a');
        $e = !$this->items->has('a');
        $f = (new Collection([1]))->doesntContain(1);
        $g = !collect([1])->contains(1);
    }
}
