<?php

final class UntypedReference
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        $r = &$x;
        $this->p[] = $x;
    }

    public function f()
    {
        return empty($this->p);
    }
}

final class UntypedForeach
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        foreach ([1] as $x) {
        }
        $this->p[] = $x;
    }

    public function f()
    {
        return empty($this->p);
    }
}

final class UntypedAssigned
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        $x = 2;
        $this->p[] = $x;
    }

    public function f()
    {
        return empty($this->p);
    }
}
-----
<?php

final class UntypedReference
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        $r = &$x;
        $this->p[] = $x;
    }

    public function f()
    {
        return $this->p === [];
    }
}

final class UntypedForeach
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        foreach ([1] as $x) {
        }
        $this->p[] = $x;
    }

    public function f()
    {
        return $this->p === [];
    }
}

final class UntypedAssigned
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        $x = 2;
        $this->p[] = $x;
    }

    public function f()
    {
        return $this->p === [];
    }
}
