<?php

class A
{
    private $first;
    public $x;

    /**
     * Doc.
     */
    private $second = 1;

    // comment
    private $third;
    private $fourth;

    public function __construct()
    {
        $this->first = 1;
        // about second
        $this->second = 2;
        if ($this->third = foo()) {
        }
        bar($this->fourth = 3, 4);
    }
}

class B
{
    private $only;
}

class C
{
    private $a;

    private $b;
}
-----
<?php

class A
{
    public $x;

    // comment
    private $third;

    public function __construct()
    {
        if ($this->third = foo()) {
        }
        bar(3, 4);
    }
}

class B
{
}

class C
{
}
