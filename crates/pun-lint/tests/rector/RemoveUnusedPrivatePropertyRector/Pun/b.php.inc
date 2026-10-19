<?php

class D
{
    private $a;
    private $b;
    private $c;

    public function run($other)
    {
        $this->a = 1;
        $other->b = 2;
        return $this->c = 3;
    }
}

class E
{
    /** @ORM\Column */
    private $x;
    private static $y;
    #[Foo]
    private $z;
}
-----
<?php

class D
{
    private $b;

    public function run($other)
    {
        $other->b = 2;
        return 3;
    }
}

class E
{
    /** @ORM\Column */
    private $x;
    private static $y;
    #[Foo]
    private $z;
}
