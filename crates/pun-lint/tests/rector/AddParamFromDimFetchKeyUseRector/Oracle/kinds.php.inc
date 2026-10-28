<?php

final class K
{
    public function a($key)
    {
        $items = [111 => 'x', 'first' => 'y'];
        return $items[$key];
    }

    public function b($key)
    {
        $items = ['x', 'y'];
        return $items[$key];
    }

    /** @param array<string, int> $items */
    public function c($key, array $items)
    {
        return $items[$key];
    }

    public function d($key, array $items)
    {
        return $items[$key];
    }

    public function e($key)
    {
        $items = ['a' => 1];
        $items2 = [2 => 1];
        echo $items2[$key];
        return $items[$key];
    }

    public function f($key)
    {
        $key = 'a';
        $items = ['a' => 1];
        return $items[$key];
    }

    private function g($key)
    {
        $items = ['a' => 1];
        return isset($items[$key]) ? $items[$key] : null;
    }

    /** @param list<string> $items */
    public function i($key, array $items)
    {
        return $items[$key];
    }

    /** @param non-empty-array<string, int> $items */
    public function j($key, array $items)
    {
        return $items[$key];
    }

    /** @var array<string, int> */
    private array $map = [];

    public function k($key)
    {
        return $this->map[$key];
    }

    /** @param int|string $key */
    public function l($key)
    {
        return $this->map[$key];
    }

    public function m($key)
    {
        $items = [1 => 'x', 'a' => 'y'];
        $items = ['a' => 'y', 1 => 'x'];

        return $items[$key];
    }

    public function n($key)
    {
        $items = ['a' => 'y', self::X => 'x'];

        return $items[$key];
    }

    const X = 5;
}

trait T
{
    public function o($key)
    {
        $items = ['a' => 1];
        return $items[$key];
    }
}

function make()
{
    return new class {
        public function p($key)
        {
            $items = ['a' => 1];
            return $items[$key];
        }
    };
}

function h($key)
{
    $items = ['a' => 1];
    return $items[$key];
}
-----
<?php

final class K
{
    public function a(int|string $key)
    {
        $items = [111 => 'x', 'first' => 'y'];
        return $items[$key];
    }

    public function b(int $key)
    {
        $items = ['x', 'y'];
        return $items[$key];
    }

    /** @param array<string, int> $items */
    public function c(string $key, array $items)
    {
        return $items[$key];
    }

    public function d($key, array $items)
    {
        return $items[$key];
    }

    public function e(string $key)
    {
        $items = ['a' => 1];
        $items2 = [2 => 1];
        echo $items2[$key];
        return $items[$key];
    }

    public function f($key)
    {
        $key = 'a';
        $items = ['a' => 1];
        return $items[$key];
    }

    private function g($key)
    {
        $items = ['a' => 1];
        return isset($items[$key]) ? $items[$key] : null;
    }

    /** @param list<string> $items */
    public function i(int $key, array $items)
    {
        return $items[$key];
    }

    /** @param non-empty-array<string, int> $items */
    public function j($key, array $items)
    {
        return $items[$key];
    }

    /** @var array<string, int> */
    private array $map = [];

    public function k(string $key)
    {
        return $this->map[$key];
    }

    /** @param int|string $key */
    public function l($key)
    {
        return $this->map[$key];
    }

    public function m(string|int $key)
    {
        $items = [1 => 'x', 'a' => 'y'];
        $items = ['a' => 'y', 1 => 'x'];

        return $items[$key];
    }

    public function n(string|int $key)
    {
        $items = ['a' => 'y', self::X => 'x'];

        return $items[$key];
    }

    const X = 5;
}

trait T
{
    public function o($key)
    {
        $items = ['a' => 1];
        return $items[$key];
    }
}

function make()
{
    return new class {
        public function p(string $key)
        {
            $items = ['a' => 1];
            return $items[$key];
        }
    };
}

function h($key)
{
    $items = ['a' => 1];
    return $items[$key];
}
