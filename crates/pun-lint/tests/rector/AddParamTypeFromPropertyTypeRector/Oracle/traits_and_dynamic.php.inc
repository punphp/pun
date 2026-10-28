<?php

namespace Probe;

trait Named
{
    private string $name = '';

    public function rename($name)
    {
        $this->name = $name;
    }
}

enum Suit
{
    case Hearts;

    public function label($label)
    {
        $copy = $label;

        return $copy;
    }
}

final class Dynamic
{
    private int $count = 0;

    private static int $total = 0;

    public function set($value, string $field)
    {
        $copy = $value;
        $this->{$field} = $value;
    }

    public function setStatic($value, string $field)
    {
        self::${$field} = $value;
    }

    public function both($value)
    {
        $this->count = $value;
        static::$total = $value;
    }
}
-----
<?php

namespace Probe;

trait Named
{
    private string $name = '';

    public function rename($name)
    {
        $this->name = $name;
    }
}

enum Suit
{
    case Hearts;

    public function label($label)
    {
        $copy = $label;

        return $copy;
    }
}

final class Dynamic
{
    private int $count = 0;

    private static int $total = 0;

    public function set($value, string $field)
    {
        $copy = $value;
        $this->{$field} = $value;
    }

    public function setStatic($value, string $field)
    {
        self::${$field} = $value;
    }

    public function both(int $value)
    {
        $this->count = $value;
        static::$total = $value;
    }
}
