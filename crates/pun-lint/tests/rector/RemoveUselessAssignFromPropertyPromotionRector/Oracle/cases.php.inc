<?php

class A
{
    public function __construct(private $std, public readonly int $ro, protected $other)
    {
        $this->std = $std;
        $this->Other = $other;
        // trailing
    }
}

class B
{
    public function __CONSTRUCT(private $std, protected $other)
    {
        // leading
        $this->std = $std;
        $THIS->Other = $Other;
    }
}

class C
{
    public function __construct(private $std)
    {
        $this->std = $std;
        $this->std = $std;
    }
}
-----
<?php

class A
{
    public function __construct(private $std, public readonly int $ro, protected $other)
    {
        $this->std = $std;
        $this->Other = $other;
        // trailing
    }
}

class B
{
    public function __CONSTRUCT(private $std, protected $other)
    {
    }
}

class C
{
    public function __construct(private $std)
    {
    }
}
