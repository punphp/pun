<?php

namespace Probe;

final class Order
{
    /** @var list<int|string|null> */
    protected array $list = [];

    /** @var array<string, null|float|int> */
    protected array $keyed = [];

    /** @var (string|int)[] */
    protected array $bracketed = [];

    /** @var array<int, Order|\Closure> */
    protected array $objects = [];

    public function run(string $a, ?int $b, int $c)
    {
        array_filter($this->list, fn ($v) => $v !== null);
        array_filter($this->keyed, fn ($v) => $v !== null);
        array_filter($this->bracketed, fn ($v) => $v !== 1);
        array_filter($this->objects, fn ($v) => $v !== null);
        array_filter([$b, $a], fn ($v) => $v !== null);
        array_filter([1 => $a, 0 => $c], fn ($v) => $v !== null);
        $x = ['z' => $c, 'a' => $a];
        array_filter($x, fn ($v) => $v !== null);
    }
}
-----
<?php

namespace Probe;

final class Order
{
    /** @var list<int|string|null> */
    protected array $list = [];

    /** @var array<string, null|float|int> */
    protected array $keyed = [];

    /** @var (string|int)[] */
    protected array $bracketed = [];

    /** @var array<int, Order|\Closure> */
    protected array $objects = [];

    public function run(string $a, ?int $b, int $c)
    {
        array_filter($this->list, fn (int|string|null $v) => $v !== null);
        array_filter($this->keyed, fn (float|int|null $v) => $v !== null);
        array_filter($this->bracketed, fn (string|int $v) => $v !== 1);
        array_filter($this->objects, fn (\Probe\Order|\Closure $v) => $v !== null);
        array_filter([$b, $a], fn (int|string|null $v) => $v !== null);
        array_filter([1 => $a, 0 => $c], fn (string|int $v) => $v !== null);
        $x = ['z' => $c, 'a' => $a];
        array_filter($x, fn (int|string $v) => $v !== null);
    }
}
