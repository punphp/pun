<?php

namespace Pun\Oracle\Override;

class Item
{
}

abstract class Base
{
    public function find(?Item $item): ?Item
    {
        return $item;
    }

    public static function make(): void
    {
    }

    protected function keep(int $a)
    {
    }

    public function named(int $a)
    {
    }

    public function defaulted(int $a = 1)
    {
    }

    public function &byRef()
    {
        static $a;
        return $a;
    }

    public function other(string $a)
    {
    }
}

class Child extends Base
{
    public function find(?Item $item): ?Item
    {
        return parent::find($item);
    }

    public static function make(): void
    {
        parent::make();
    }

    public function keep(int $a)
    {
        parent::keep($a);
    }

    public function named(int $a)
    {
        parent::named(a: $a);
    }

    public function defaulted(int $a = 1)
    {
        parent::defaulted($a);
    }

    public function other(int $a)
    {
        parent::other($a);
    }
}

class StaticMismatch extends Base
{
    public function make(): void
    {
        parent::make();
    }
}
-----
<?php

namespace Pun\Oracle\Override;

class Item
{
}

abstract class Base
{
    public function find(?Item $item): ?Item
    {
        return $item;
    }

    public static function make(): void
    {
    }

    protected function keep(int $a)
    {
    }

    public function named(int $a)
    {
    }

    public function defaulted(int $a = 1)
    {
    }

    public function &byRef()
    {
        static $a;
        return $a;
    }

    public function other(string $a)
    {
    }
}

class Child extends Base
{
    public function keep(int $a)
    {
        parent::keep($a);
    }

    public function named(int $a)
    {
        parent::named(a: $a);
    }

    public function defaulted(int $a = 1)
    {
        parent::defaulted($a);
    }

    public function other(int $a)
    {
        parent::other($a);
    }
}

class StaticMismatch extends Base
{
    public function make(): void
    {
        parent::make();
    }
}
