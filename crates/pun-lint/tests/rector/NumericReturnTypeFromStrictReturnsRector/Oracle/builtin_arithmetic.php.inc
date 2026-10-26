<?php

final class BuiltinArithmetic
{
    public function scaled()
    {
        return microtime(true) * 1000;
    }

    public function negated()
    {
        return -microtime(true);
    }

    public function doubled()
    {
        return time() * 2;
    }

    public function position(string $s)
    {
        return strpos($s, 'a') + 1;
    }

    public function merged(array $a)
    {
        return array_merge($a, []) + ['x' => 1];
    }

    public function counted(array $a)
    {
        return count(array_filter($a)) * 2;
    }

    public function time()
    {
        return microtime(true);
    }

    public function absolute(int $i)
    {
        return abs($i);
    }

    public function filtered(array $a)
    {
        return array_filter($a) * 2;
    }

    public function created()
    {
        return date_create() - 1;
    }

    public function largest(array $a)
    {
        return max($a) * 2;
    }

    public function smallest(int $a, int $b)
    {
        return min($a, $b) * 2;
    }

    public function scaledAbsolute(int $i)
    {
        return abs($i) * 2;
    }

    public function negatedAbsolute(int $i)
    {
        return -abs($i);
    }

    public function negatedTime()
    {
        return -time();
    }
}
-----
<?php

final class BuiltinArithmetic
{
    public function scaled()
    {
        return microtime(true) * 1000;
    }

    public function negated()
    {
        return -microtime(true);
    }

    public function doubled(): int
    {
        return time() * 2;
    }

    public function position(string $s): int
    {
        return strpos($s, 'a') + 1;
    }

    public function merged(array $a)
    {
        return array_merge($a, []) + ['x' => 1];
    }

    public function counted(array $a): int
    {
        return count(array_filter($a)) * 2;
    }

    public function time(): float
    {
        return microtime(true);
    }

    public function absolute(int $i): int
    {
        return abs($i);
    }

    public function filtered(array $a)
    {
        return array_filter($a) * 2;
    }

    public function created()
    {
        return date_create() - 1;
    }

    public function largest(array $a)
    {
        return max($a) * 2;
    }

    public function smallest(int $a, int $b)
    {
        return min($a, $b) * 2;
    }

    public function scaledAbsolute(int $i)
    {
        return abs($i) * 2;
    }

    public function negatedAbsolute(int $i)
    {
        return -abs($i);
    }

    public function negatedTime(): int
    {
        return -time();
    }
}
