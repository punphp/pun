<?php

trait EmptyTrait
{
    public function f(array $a)
    {
        return empty($a);
    }
}

function local()
{
    $v = [1];
    return empty($v);
}

$anon = new class {
    private array $p = [];

    public function f()
    {
        return empty($this->p);
    }
};

final class ElementWrites
{
    private array $p = [];

    public function add(int $x)
    {
        $this->p[] = 1;
        $this->p[] = ${'x'};
    }

    public function f()
    {
        return empty($this->p);
    }
}

final class ReferenceWrite
{
    private array $p = [];

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

final class ForeachWrite
{
    private array $p = [];

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

final class GlobalWrite
{
    private array $p = [];

    public function add(int $x)
    {
        global $y;
        $this->p[] = $x;
    }

    public function f()
    {
        return empty($this->p);
    }
}
-----
<?php

trait EmptyTrait
{
    public function f(array $a)
    {
        return $a === [];
    }
}

function local()
{
    $v = [1];
    return $v === [];
}

$anon = new class {
    private array $p = [];

    public function f()
    {
        return $this->p === [];
    }
};

final class ElementWrites
{
    private array $p = [];

    public function add(int $x)
    {
        $this->p[] = 1;
        $this->p[] = ${'x'};
    }

    public function f()
    {
        return $this->p === [];
    }
}

final class ReferenceWrite
{
    private array $p = [];

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

final class ForeachWrite
{
    private array $p = [];

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

final class GlobalWrite
{
    private array $p = [];

    public function add(int $x)
    {
        global $y;
        $this->p[] = $x;
    }

    public function f()
    {
        return $this->p === [];
    }
}
