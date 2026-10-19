<?php

final class Oracle
{
    private array $items = [];

    private $other;

    public function run(array $list, $mixed)
    {
        $a = ! empty($list) ? $list['key'] : null;
        $b = !empty($this->items) ? $this->items[0] : $list[1] ?? 'x';
        $c = (! empty($this->items) ? $this->items[0] : 'default') . 'x';
        $d = ! empty($mixed) ? $mixed[0] : 'default';
        $e = ! empty($list) ? $this->items[0] : 'default';
        $f = ! empty($list) ?: 'default';
        $g = ! isset($list) ? $list[0] : 'default';
        $h = - $list ? $list[0] : 'default';
        $i = ! empty($list) ? $list->x : 'default';
        $j = empty($list) ? $list[0] : 'default';
        return $a . $b . $c . $d . $e . $f . $g . $h . $i . $j;
    }
}
-----
<?php

final class Oracle
{
    private array $items = [];

    private $other;

    public function run(array $list, $mixed)
    {
        $a = $list['key'] ?? null;
        $b = $this->items[0] ?? $list[1] ?? 'x';
        $c = ($this->items[0] ?? 'default') . 'x';
        $d = ! empty($mixed) ? $mixed[0] : 'default';
        $e = ! empty($list) ? $this->items[0] : 'default';
        $f = ! empty($list) ?: 'default';
        $g = ! isset($list) ? $list[0] : 'default';
        $h = - $list ? $list[0] : 'default';
        $i = ! empty($list) ? $list->x : 'default';
        $j = empty($list) ? $list[0] : 'default';
        return $a . $b . $c . $d . $e . $f . $g . $h . $i . $j;
    }
}
