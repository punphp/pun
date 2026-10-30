<?php

namespace App;

final class NativeProbe
{
    public function a(string $s)
    {
        return strlen($s);
    }

    public function b(array $a)
    {
        return count($a);
    }

    public function c(string $s)
    {
        return explode(',', $s);
    }

    public function d(string $s)
    {
        return json_encode($s);
    }

    public function e(array $a)
    {
        return array_map(fn ($x) => $x, $a);
    }

    public function f(string $s)
    {
        return str_replace('a', 'b', $s);
    }

    public function g($x)
    {
        return max(1, $x);
    }

    public function h(string $s)
    {
        return preg_replace('/a/', 'b', $s);
    }

    public function i(\DateTime $d)
    {
        return $d->format('Y');
    }

    public function j(array $a)
    {
        return array_pop($a);
    }

    public function k(string $s)
    {
        return strpos($s, 'a');
    }

    public function l(string $s)
    {
        if ($s) {
            return implode(',', [$s]);
        }
        return sprintf('%s', $s);
    }

    public function m(array $a)
    {
        return array_filter($a);
    }

    public function n($x)
    {
        return is_string($x);
    }

    public function o(string $s)
    {
        return \Closure::fromCallable('strlen');
    }

    public function p(string $s)
    {
        return array_keys([$s => 1]);
    }

    public function q(\ArrayObject $o)
    {
        return $o->getIterator();
    }

    public function r(string $s)
    {
        return json_decode($s, true);
    }
}
-----
<?php

namespace App;

final class NativeProbe
{
    public function a(string $s): int
    {
        return strlen($s);
    }

    public function b(array $a): int
    {
        return count($a);
    }

    public function c(string $s): array
    {
        return explode(',', $s);
    }

    public function d(string $s): string|false
    {
        return json_encode($s);
    }

    public function e(array $a): array
    {
        return array_map(fn ($x) => $x, $a);
    }

    public function f(string $s): string
    {
        return str_replace('a', 'b', $s);
    }

    public function g($x)
    {
        return max(1, $x);
    }

    public function h(string $s): ?string
    {
        return preg_replace('/a/', 'b', $s);
    }

    public function i(\DateTime $d): string
    {
        return $d->format('Y');
    }

    public function j(array $a)
    {
        return array_pop($a);
    }

    public function k(string $s): int|false
    {
        return strpos($s, 'a');
    }

    public function l(string $s): string
    {
        if ($s) {
            return implode(',', [$s]);
        }
        return sprintf('%s', $s);
    }

    public function m(array $a): array
    {
        return array_filter($a);
    }

    public function n($x): bool
    {
        return is_string($x);
    }

    public function o(string $s): \Closure
    {
        return \Closure::fromCallable('strlen');
    }

    public function p(string $s): array
    {
        return array_keys([$s => 1]);
    }

    public function q(\ArrayObject $o): \ArrayIterator
    {
        return $o->getIterator();
    }

    public function r(string $s)
    {
        return json_decode($s, true);
    }
}
