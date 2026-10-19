<?php

namespace App;

final class Mixed
{
    /**
     * @param int $a
     * @param int $b
     */
    public function __construct(
        private int $a,
        private readonly int $b,
        protected int $c,
        private int $d,
        private int &$e,
        private int $f = 1,
    ) {
        echo $d;
    }

    public function read()
    {
        return self::$f ?? $this?->a;
    }
}

final class AllGone
{
    public function __construct(private $x, private $y)
    {
    }
}

#[\Attribute]
final class Marked
{
    public function __construct(private $x)
    {
    }
}

final class Magic
{
    public function __construct(private $x)
    {
    }

    public function __get($name)
    {
    }
}

final class Dynamic
{
    public function __construct(private $x)
    {
    }

    public function read($name)
    {
        return $this->$name;
    }
}

final class Vars
{
    public function __construct(private $x)
    {
    }

    public function all()
    {
        return get_object_vars($this);
    }
}

$object = new class (1) {
    public function __construct(private $x)
    {
    }
};
-----
<?php

namespace App;

final class Mixed
{
    /**
     * @param int $a
     */
    public function __construct(
        private int $a,
        protected int $c,
        int $d,
        private int $f = 1,
    ) {
        echo $d;
    }

    public function read()
    {
        return self::$f ?? $this?->a;
    }
}

final class AllGone
{
    public function __construct()
    {
    }
}

#[\Attribute]
final class Marked
{
    public function __construct(private $x)
    {
    }
}

final class Magic
{
    public function __construct(private $x)
    {
    }

    public function __get($name)
    {
    }
}

final class Dynamic
{
    public function __construct(private $x)
    {
    }

    public function read($name)
    {
        return $this->$name;
    }
}

final class Vars
{
    public function __construct(private $x)
    {
    }

    public function all()
    {
        return get_object_vars($this);
    }
}

$object = new class (1) {
    public function __construct()
    {
    }
};
