<?php

namespace Probe;

class Model
{
}

final class Box
{
    /** @var list<int|string> */
    public array $items = [];

    /** @var Box */
    public $box;

    /** @var array<int, int[]|string> */
    protected array $nested = [];

    /** @var array<int, 'a'|int> */
    protected array $literal = [];

    /** @var array<int, string> */
    protected array $single = [];

    /** @var array<int, Model&Missing> */
    protected array $unknown = [];

    public function run(Box $other, int $a, string $b)
    {
        array_filter($this->box->items, fn ($v) => $v !== 1);
        array_filter($other->items, fn ($v) => $v !== 1);
        array_filter($this->nested, fn ($v) => $v !== 1);
        array_filter($this->literal, fn ($v) => $v !== 1);
        array_filter($this->single, fn ($v) => $v !== '');
        array_filter($this->unknown, fn ($v) => $v !== null);
        array_filter(array($b, $a), fn ($v) => $v !== 1);
        $y = ['k' => $a];
        $y['l'] = $a;
        array_filter($y, fn ($v) => $v !== 1);
        array_filter([...[$b], $a], fn ($v) => $v !== 1);
    }
}
-----
<?php

namespace Probe;

class Model
{
}

final class Box
{
    /** @var list<int|string> */
    public array $items = [];

    /** @var Box */
    public $box;

    /** @var array<int, int[]|string> */
    protected array $nested = [];

    /** @var array<int, 'a'|int> */
    protected array $literal = [];

    /** @var array<int, string> */
    protected array $single = [];

    /** @var array<int, Model&Missing> */
    protected array $unknown = [];

    public function run(Box $other, int $a, string $b)
    {
        array_filter($this->box->items, fn (int|string $v) => $v !== 1);
        array_filter($other->items, fn (int|string $v) => $v !== 1);
        array_filter($this->nested, fn (string|array $v) => $v !== 1);
        array_filter($this->literal, fn (int|string $v) => $v !== 1);
        array_filter($this->single, fn (string $v) => $v !== '');
        array_filter($this->unknown, fn (\Probe\Model&\Probe\Missing $v) => $v !== null);
        array_filter(array($b, $a), fn (string|int $v) => $v !== 1);
        $y = ['k' => $a];
        $y['l'] = $a;
        array_filter($y, fn (int $v) => $v !== 1);
        array_filter([...[$b], $a], fn (string|int $v) => $v !== 1);
    }
}
