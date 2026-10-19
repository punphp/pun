<?php

class Base
{
    public static int $count = 0;
}

class Item
{
    public string $name = '';
}

final class Fetches extends Base
{
    public static int $total = 0;

    private mixed $any;

    private Item $item;

    private int $a = 0;

    public function __construct()
    {
        $this->item = new Item();
    }

    public function dynamic($x)
    {
        return $this->{$x};
    }

    public function other()
    {
        return $this->item->name;
    }

    public function untyped($o)
    {
        return $o->name;
    }

    public function staticVariable($x)
    {
        return self::$$x;
    }

    public function parentStatic()
    {
        return parent::$count;
    }

    public function named()
    {
        return Fetches::$total;
    }

    public function dynamicClass($c)
    {
        return $c::$total;
    }

    public function anyType()
    {
        return $this->any;
    }

    public function withVoid($x)
    {
        if ($x) {
            return;
        }
        return $this->a;
    }
}
-----
<?php

class Base
{
    public static int $count = 0;
}

class Item
{
    public string $name = '';
}

final class Fetches extends Base
{
    public static int $total = 0;

    private mixed $any;

    private Item $item;

    private int $a = 0;

    public function __construct()
    {
        $this->item = new Item();
    }

    public function dynamic($x)
    {
        return $this->{$x};
    }

    public function other(): string
    {
        return $this->item->name;
    }

    public function untyped($o)
    {
        return $o->name;
    }

    public function staticVariable($x)
    {
        return self::$$x;
    }

    public function parentStatic(): int
    {
        return parent::$count;
    }

    public function named(): int
    {
        return Fetches::$total;
    }

    public function dynamicClass($c)
    {
        return $c::$total;
    }

    public function anyType()
    {
        return $this->any;
    }

    public function withVoid($x)
    {
        if ($x) {
            return;
        }
        return $this->a;
    }
}
