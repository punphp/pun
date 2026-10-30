<?php

namespace Pun\Oracle\ParamFromProperty;

final class Kinds
{
    private mixed $anything;
    private int|string $id;
    private static ?\DateTimeInterface $at = null;
    private \Closure $callback;
    private self $me;
    private int $a;
    private string $b;

    public function explicitMixed($anything)
    {
        $this->anything = $anything;
    }

    public function union($id)
    {
        $this->id = $id;
    }

    public function staticProperty($at)
    {
        static::$at = $at;
    }

    public function inClosure($callback)
    {
        $set = function () use ($callback) {
            $this->callback = $callback;
        };
    }

    public function self($me)
    {
        $this->me = $me;
    }

    public function two($value)
    {
        $this->a = $value;
        $this->b = $value;
    }

    public function sameTwice($value)
    {
        $this->a = $value;
        $this->a = $value;
    }

    public function reassigned($value)
    {
        $value = 1;
        $this->a = $value;
    }
}
-----
<?php

namespace Pun\Oracle\ParamFromProperty;

final class Kinds
{
    private mixed $anything;
    private int|string $id;
    private static ?\DateTimeInterface $at = null;
    private \Closure $callback;
    private self $me;
    private int $a;
    private string $b;

    public function explicitMixed($anything)
    {
        $this->anything = $anything;
    }

    public function union(int|string $id)
    {
        $this->id = $id;
    }

    public function staticProperty(?\DateTimeInterface $at)
    {
        static::$at = $at;
    }

    public function inClosure(\Closure $callback)
    {
        $set = function () use ($callback) {
            $this->callback = $callback;
        };
    }

    public function self(\Pun\Oracle\ParamFromProperty\Kinds $me)
    {
        $this->me = $me;
    }

    public function two(int|string $value)
    {
        $this->a = $value;
        $this->b = $value;
    }

    public function sameTwice(int $value)
    {
        $this->a = $value;
        $this->a = $value;
    }

    public function reassigned($value)
    {
        $value = 1;
        $this->a = $value;
    }
}
