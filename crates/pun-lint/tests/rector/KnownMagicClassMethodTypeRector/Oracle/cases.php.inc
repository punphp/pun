<?php

namespace App;

final class Magic
{
    public function __call($name, $arguments)
    {
    }

    public static function __callStatic($name, array $arguments)
    {
    }

    public function __get(string $name)
    {
    }

    public function __set($name, &$value)
    {
    }

    public function __isset($name): bool
    {
        return false;
    }

    public function __unset(...$name)
    {
    }
}

final class Child extends \ArrayObject
{
    public function __get($name)
    {
    }
}

trait InTrait
{
    public function __get($name)
    {
    }
}

$anonymous = new class {
    private function __get($name)
    {
    }
};
-----
<?php

namespace App;

final class Magic
{
    public function __call(string $name, array $arguments)
    {
    }

    public static function __callStatic(string $name, array $arguments)
    {
    }

    public function __get(string $name): mixed
    {
    }

    public function __set(string $name, mixed &$value)
    {
    }

    public function __isset(string $name): bool
    {
        return false;
    }

    public function __unset(string ...$name)
    {
    }
}

final class Child extends \ArrayObject
{
    public function __get(string $name): mixed
    {
    }
}

trait InTrait
{
    public function __get($name)
    {
    }
}

$anonymous = new class {
    private function __get(string $name): mixed
    {
    }
};
