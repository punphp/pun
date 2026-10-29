<?php

namespace Pun\Oracle\DynamicProperties;

class Child extends \Exception
{
    public function set()
    {
        $this->message = 'a';
        $this->own = 2;
    }
}

class Promoted
{
    public function __construct(private int $p)
    {
    }

    public function set()
    {
        $this->p = 1;
        $this->q = 2;
    }
}

class WithConstant
{
    const A = 1;

    public function set()
    {
        $this->z = self::A;
    }
}
-----
<?php

namespace Pun\Oracle\DynamicProperties;

class Child extends \Exception
{
    /**
     * @var int
     */
    public $own;
    public function set()
    {
        $this->message = 'a';
        $this->own = 2;
    }
}

class Promoted
{
    /**
     * @var int
     */
    public $q;
    public function __construct(private int $p)
    {
    }

    public function set()
    {
        $this->p = 1;
        $this->q = 2;
    }
}

class WithConstant
{
    /**
     * @var int
     */
    public $z;
    const A = 1;

    public function set()
    {
        $this->z = self::A;
    }
}
