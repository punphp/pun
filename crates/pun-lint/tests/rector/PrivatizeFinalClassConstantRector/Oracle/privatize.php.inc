<?php

final class A implements \Countable
{
    protected const ONE = 1;
    protected const TWO = 2, THREE = 3;
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

final class WithTrait
{
    use T;

    protected $x;
}
-----
<?php

final class A implements \Countable
{
    private const ONE = 1;
    protected const TWO = 2, THREE = 3;
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

final class WithTrait
{
    use T;

    protected $x;
}
