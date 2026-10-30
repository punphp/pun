<?php

namespace Probe;

trait Values
{
    public $value;

    public function forms(bool $flag, array $list)
    {
        $value = $this->value;
        $a = $flag ? $value : null;
        $b = $value ?? $list;
        $c = ($value);
        $d = array($value);
        $e = [$value];
        $f = array_keys($list);
        $g = $flag ?: $value;

        return array_any($a, fn ($x) => $x !== null)
            || array_any($b, fn ($x) => $x !== null)
            || array_any($c, fn ($x) => $x !== null)
            || array_any($d, fn ($x) => $x !== null)
            || array_any($e, fn ($x) => $x !== null)
            || array_any($f, fn ($x) => $x !== null)
            || array_any($g, fn ($x) => $x !== null);
    }
}
-----
<?php

namespace Probe;

trait Values
{
    public $value;

    public function forms(bool $flag, array $list)
    {
        $value = $this->value;
        $a = $flag ? $value : null;
        $b = $value ?? $list;
        $c = ($value);
        $d = array($value);
        $e = [$value];
        $f = array_keys($list);
        $g = $flag ?: $value;

        return array_any($a, fn ($x) => $x !== null)
            || array_any($b, fn ($x) => $x !== null)
            || array_any($c, fn ($x) => $x !== null)
            || array_any($d, fn ($x) => $x !== null)
            || array_any($e, fn ($x) => $x !== null)
            || array_any($f, fn (int|string $x) => $x !== null)
            || array_any($g, fn ($x) => $x !== null);
    }
}
