<?php

$anonymous = new class {
    private $value;

    public function __construct()
    {
        $this->value = 1;
    }
};

class NoConstructor
{
    private $value;
}

abstract class AbstractConstructor
{
    private $value;

    abstract public function __construct();
}

class Statements
{
    public $hooked {
        get => 1;
    }

    private $a;

    private $b;

    private $c;

    private $d;

    private $e;

    private $f;

    private $g;

    private $h;

    public function __construct($x)
    {
        foo();
        $this->a += 1;
        $this->b = &$x;
        $other->c = 1;
        $this->{$x} = 1;
        $self->d = 1;
        $this->e = 1;
        // second
        $this->e = 2;
        $this->f = 1;
        $this->g = 1;
        $this->undeclared = 1;
    }
}
-----
<?php

$anonymous = new class {
    private $value = 1;

    public function __construct()
    {
    }
};

class NoConstructor
{
    private $value;
}

abstract class AbstractConstructor
{
    private $value;

    abstract public function __construct();
}

class Statements
{
    public $hooked {
        get => 1;
    }

    private $a;

    private $b;

    private $c;

    private $d;

    // second
    private $e = 2;

    private $f = 1;

    private $g = 1;

    private $h;

    public function __construct($x)
    {
        foo();
        $this->a += 1;
        $this->b = &$x;
        $other->c = 1;
        $this->{$x} = 1;
        $self->d = 1;
        $this->undeclared = 1;
    }
}
