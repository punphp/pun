<?php

namespace App;

final class Items
{
    public function bools(bool $b)
    {
        if ($b) {
            return [true, false];
        }

        return 1;
    }

    public function ints(bool $b, int $i)
    {
        if ($b) {
            return [1, $i];
        }

        return 'a';
    }

    public function floats(bool $b, float $x)
    {
        if ($b) {
            return [1.5, $x];
        }

        return 'a';
    }

    public function strings(bool $b, string $s)
    {
        if ($b) {
            return ['a', $s];
        }

        return 1;
    }

    public function merged(bool $b, bool $c, array $a)
    {
        if ($b) {
            return $c ? $a : array_values($a);
        }

        return $c ? $a : ($b ? array_values($a) : 1);
    }

    public function mapped(bool $b, array $a)
    {
        if ($b) {
            return 'a';
        }

        return $b ? array_map(fn ($x) => $x, $a) : 1;
    }
}
-----
<?php

namespace App;

final class Items
{
    public function bools(bool $b): array|int
    {
        if ($b) {
            return [true, false];
        }

        return 1;
    }

    public function ints(bool $b, int $i): array|string
    {
        if ($b) {
            return [1, $i];
        }

        return 'a';
    }

    public function floats(bool $b, float $x): array|string
    {
        if ($b) {
            return [1.5, $x];
        }

        return 'a';
    }

    public function strings(bool $b, string $s): array|int
    {
        if ($b) {
            return ['a', $s];
        }

        return 1;
    }

    public function merged(bool $b, bool $c, array $a): array|int
    {
        if ($b) {
            return $c ? $a : array_values($a);
        }

        return $c ? $a : ($b ? array_values($a) : 1);
    }

    public function mapped(bool $b, array $a): string|int
    {
        if ($b) {
            return 'a';
        }

        return $b ? array_map(fn ($x) => $x, $a) : 1;
    }
}
