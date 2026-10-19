<?php

function by_reference(array &$a)
{
    return empty($a);
}

class A
{
    /** @var array<int, string> */
    private $generic = [];

    /** @var int|array */
    private $union = [];

    /** The items. */
    private $noVar = [];

    /** @var array{a: int} */
    private $shape = [];

    private static array $static = [];

    public array $hooked {
        get => [];
    }

    public function run($name)
    {
        $a = empty($this->$name);
        $b = empty(self::$static);
        $c = empty(static::$static);
        $d = empty(self::$$name);
        $e = empty($this->generic);
        $f = empty($this->union);
        $g = empty($this->noVar);
        $h = empty($this->hooked);
        $i = empty($this->shape);
    }

    public function write($value, array $typed, $name)
    {
        $this->generic['key'] = $value;
        self::$static[] = $typed;
        static::$static = [];
        self::$$name = [];
        $this->shape[] = $$name;
    }
}

function outside()
{
    return empty($this->items);
}

function locals()
{
    $closure = function () {
        $items = [];
        return empty($items);
    };
    $late = foo($items);
    if ($late) {
        $items = [];
    }
    return empty($items);
}

function never_assigned()
{
    echo 1;
    return empty($items);
}

$r = empty($undefined);
-----
<?php

function by_reference(array &$a)
{
    return $a === [];
}

class A
{
    /** @var array<int, string> */
    private $generic = [];

    /** @var int|array */
    private $union = [];

    /** The items. */
    private $noVar = [];

    /** @var array{a: int} */
    private $shape = [];

    private static array $static = [];

    public array $hooked {
        get => [];
    }

    public function run($name)
    {
        $a = empty($this->$name);
        $b = self::$static === [];
        $c = static::$static === [];
        $d = empty(self::$$name);
        $e = empty($this->generic);
        $f = empty($this->union);
        $g = empty($this->noVar);
        $h = $this->hooked === [];
        $i = empty($this->shape);
    }

    public function write($value, array $typed, $name)
    {
        $this->generic['key'] = $value;
        self::$static[] = $typed;
        static::$static = [];
        self::$$name = [];
        $this->shape[] = $$name;
    }
}

function outside()
{
    return empty($this->items);
}

function locals()
{
    $closure = function () {
        $items = [];
        return $items === [];
    };
    $late = foo($items);
    if ($late) {
        $items = [];
    }
    return empty($items);
}

function never_assigned()
{
    echo 1;
    return empty($items);
}

$r = empty($undefined);
