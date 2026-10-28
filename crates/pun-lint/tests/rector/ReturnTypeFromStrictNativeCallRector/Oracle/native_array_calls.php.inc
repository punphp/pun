<?php

final class NativeProbe2
{
    public function a(array $a)
    {
        return array_map(fn ($x) => $x, $a);
    }

    public function b(array $a)
    {
        return array_map(fn (int $x): int => $x, $a);
    }

    public function c(array $a)
    {
        return array_map('strval', $a);
    }

    public function d(array $a)
    {
        return array_values($a);
    }

    public function e(array $a)
    {
        return array_filter($a, fn ($x) => $x);
    }

    public function f(array $a)
    {
        return array_map(function ($x) { return $x; }, $a);
    }
}
-----
<?php

final class NativeProbe2
{
    public function a(array $a): array
    {
        return array_map(fn ($x) => $x, $a);
    }

    public function b(array $a): array
    {
        return array_map(fn (int $x): int => $x, $a);
    }

    public function c(array $a): array
    {
        return array_map('strval', $a);
    }

    public function d(array $a): array
    {
        return array_values($a);
    }

    public function e(array $a): array
    {
        return array_filter($a, fn ($x) => $x);
    }

    public function f(array $a): array
    {
        return array_map(function ($x) { return $x; }, $a);
    }
}
