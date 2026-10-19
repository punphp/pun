<?php

final class A implements \Countable
{
    protected const ONE = 1;
    protected const TWO = 2, THREE = 3;
    final protected const FOUR = 4;
    static protected $a;
    protected ?int $b = null, $c;
    protected   readonly int $d;
    #[Attr] protected int $e;

    public function __construct(protected readonly int $x, protected $y = 1, public $z = 2)
    {
    }

    static protected function one()
    {
    }

    protected static function createComponentFoo()
    {
    }

    protected function __clone()
    {
    }

    public function count(): int
    {
        return 0;
    }
}

class NotFinal
{
    protected $x;
}
-----
<?php

final class A implements \Countable
{
    protected const ONE = 1;
    protected const TWO = 2, THREE = 3;
    final protected const FOUR = 4;
    private static $a;
    protected ?int $b = null, $c;
    private readonly int $d;
    #[Attr] private int $e;

    public function __construct(private readonly int $x, private $y = 1, public $z = 2)
    {
    }

    static protected function one()
    {
    }

    protected static function createComponentFoo()
    {
    }

    protected function __clone()
    {
    }

    public function count(): int
    {
        return 0;
    }
}

class NotFinal
{
    protected $x;
}
