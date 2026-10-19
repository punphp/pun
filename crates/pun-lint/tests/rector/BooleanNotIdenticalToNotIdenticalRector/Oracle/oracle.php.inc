<?php

class Oracle
{
    public function run(bool $a, bool $b, $c)
    {
        $x = !($a === $b) . 'x';
        $y = (! $a) === $b && !$c === $b;
        $z = !((!$a) === $b);
        $w = !($a === $c) || -$a === $b || $a === $b || (bool) $c === $b;
        return [!($a === $b), (!$a === $b)];
    }
}
-----
<?php

class Oracle
{
    public function run(bool $a, bool $b, $c)
    {
        $x = ($a !== $b) . 'x';
        $y = $a !== $b && $c !== $b;
        $z = !$a !== $b;
        $w = !($a === $c) || -$a === $b || $a === $b || (bool) $c === $b;
        return [$a !== $b, ($a !== $b)];
    }
}
