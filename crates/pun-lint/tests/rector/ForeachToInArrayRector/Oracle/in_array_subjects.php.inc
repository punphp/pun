<?php

function other_object($o, $x)
{
    foreach ($o->items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function null_default(array $items = null, $x = 1)
{
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function null_constant(array $items = NULL, $x = 1)
{
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function not_array($x)
{
    $items = [1];
    $items = foo();
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function destructured_other($x)
{
    $items = [1];
    [$y] = [[1]];
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function incremented($x)
{
    $items = [1];
    ++$items;
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function postfix($x)
{
    $items = [1];
    $items--;
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function foreach_key($x)
{
    $items = [1];
    foreach ([[1]] as $k => $v) {
    }
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function globals($x)
{
    $items = [1];
    global $items;
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function statics($x)
{
    $items = [1];
    static $other;
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function unsets($x)
{
    $items = [1];
    unset($items);
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function variable_variable($x)
{
    $items = [1];
    $$x = 1;
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function closure($x)
{
    $items = [1];
    $f = function () use ($items) {
    $items = 1;
    };
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function nested($x)
{
    $items = [1];
    function nested_inner()
    {
    $items = 1;
    }
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function nested_array($x)
{
    $items = [[1]];
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

$f = function (array $items, $x) {
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
};

$anon = new class {
    private array $items = [];

    public function has($x)
    {
        foreach ($this->items as $item) {
            if ($item === $x) {
                return true;
            }
        }
        return false;
    }
};

trait HasItems
{
    private array $items = [];

    public function has($x)
    {
        foreach ($this->items as $item) {
            if ($item === $x) {
                return true;
            }
        }
        return false;
    }
}

enum Items
{
    public function has($x)
    {
        foreach ($this->items as $item) {
            if ($item === $x) {
                return true;
            }
        }
        return false;
    }
}

final class Promoted
{
    public function __construct(private array $items, private $other)
    {
    }

    public function has($x)
    {
        foreach ($this->items as $item) {
            if ($item === $x) {
                return true;
            }
        }
        return false;
    }

    public function hasOther($x)
    {
        foreach ($this->{$x} as $item) {
            if ($item === $x) {
                return true;
            }
        }
        return false;
    }
}
-----
<?php

function other_object($o, $x)
{
    foreach ($o->items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function null_default(array $items = null, $x = 1)
{
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function null_constant(array $items = NULL, $x = 1)
{
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function not_array($x)
{
    $items = [1];
    $items = foo();
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function destructured_other($x)
{
    $items = [1];
    [$y] = [[1]];
    return in_array($x, $items, true);
}

function incremented($x)
{
    $items = [1];
    ++$items;
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function postfix($x)
{
    $items = [1];
    $items--;
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function foreach_key($x)
{
    $items = [1];
    foreach ([[1]] as $k => $v) {
    }
    return in_array($x, $items, true);
}

function globals($x)
{
    $items = [1];
    global $items;
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function statics($x)
{
    $items = [1];
    static $other;
    return in_array($x, $items, true);
}

function unsets($x)
{
    $items = [1];
    unset($items);
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function variable_variable($x)
{
    $items = [1];
    $$x = 1;
    return in_array($x, $items, true);
}

function closure($x)
{
    $items = [1];
    $f = function () use ($items) {
    $items = 1;
    };
    return in_array($x, $items, true);
}

function nested($x)
{
    $items = [1];
    function nested_inner()
    {
    $items = 1;
    }
    return in_array($x, $items, true);
}

function nested_array($x)
{
    $items = [[1]];
    return in_array($x, $items, true);
}

$f = function (array $items, $x) {
    return in_array($x, $items, true);
};

$anon = new class {
    private array $items = [];

    public function has($x)
    {
        return in_array($x, $this->items, true);
    }
};

trait HasItems
{
    private array $items = [];

    public function has($x)
    {
        foreach ($this->items as $item) {
            if ($item === $x) {
                return true;
            }
        }
        return false;
    }
}

enum Items
{
    public function has($x)
    {
        foreach ($this->items as $item) {
            if ($item === $x) {
                return true;
            }
        }
        return false;
    }
}

final class Promoted
{
    public function __construct(private array $items, private $other)
    {
    }

    public function has($x)
    {
        return in_array($x, $this->items, true);
    }

    public function hasOther($x)
    {
        foreach ($this->{$x} as $item) {
            if ($item === $x) {
                return true;
            }
        }
        return false;
    }
}
