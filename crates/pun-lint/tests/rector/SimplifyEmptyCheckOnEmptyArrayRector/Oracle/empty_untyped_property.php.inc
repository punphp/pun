<?php

final class UntypedLiteral
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        $this->p[] = 1;
    }

    public function f()
    {
        return empty($this->p);
    }
}

final class UntypedIndirect
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        $this->p[] = ${'x'};
    }

    public function f()
    {
        return empty($this->p);
    }
}

final class UntypedParam
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        $this->p[] = $x;
    }

    public function f()
    {
        return empty($this->p);
    }
}

final class UntypedGlobal
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        global $x;
        $this->p[] = $x;
    }

    public function f()
    {
        return empty($this->p);
    }
}

final class UntypedStatic
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        static $y;
        $this->p[] = $x;
    }

    public function f()
    {
        return empty($this->p);
    }
}
-----
<?php

final class UntypedLiteral
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        $this->p[] = 1;
    }

    public function f()
    {
        return $this->p === [];
    }
}

final class UntypedIndirect
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        $this->p[] = ${'x'};
    }

    public function f()
    {
        return empty($this->p);
    }
}

final class UntypedParam
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        $this->p[] = $x;
    }

    public function f()
    {
        return $this->p === [];
    }
}

final class UntypedGlobal
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        global $x;
        $this->p[] = $x;
    }

    public function f()
    {
        return empty($this->p);
    }
}

final class UntypedStatic
{
    /** @var array */
    private $p = [];

    public function add(int $x)
    {
        static $y;
        $this->p[] = $x;
    }

    public function f()
    {
        return $this->p === [];
    }
}
