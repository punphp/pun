<?php

namespace App;

final class MergeProbe
{
    public function a(bool $b, array $a)
    {
        if ($b) {
            return [];
        }

        return $b ? 'a' : array_map(fn ($x) => $x, $a);
    }

    public function c(bool $b, array $a)
    {
        if ($b) {
            return [];
        }

        return match (true) {
            $b => array_map(fn ($x) => $x, $a),
            default => 1,
        };
    }

    public function d(bool $b, array $a)
    {
        if ($b) {
            return [];
        }

        return array_map(fn ($x) => $x, $a) ?: 1;
    }
}
-----
<?php

namespace App;

final class MergeProbe
{
    public function a(bool $b, array $a): array
    {
        if ($b) {
            return [];
        }

        return $b ? 'a' : array_map(fn ($x) => $x, $a);
    }

    public function c(bool $b, array $a): array|int
    {
        if ($b) {
            return [];
        }

        return match (true) {
            $b => array_map(fn ($x) => $x, $a),
            default => 1,
        };
    }

    public function d(bool $b, array $a)
    {
        if ($b) {
            return [];
        }

        return array_map(fn ($x) => $x, $a) ?: 1;
    }
}
