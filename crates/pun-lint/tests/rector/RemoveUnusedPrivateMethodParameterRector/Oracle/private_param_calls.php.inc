<?php

class Base
{
    public function run($a, $b)
    {
    }
}

final class Child extends Base
{
    public function go()
    {
        $c = parent::run(...);
        return $this->run(1, 2);
    }

    private function run($a, $b)
    {
        return $a;
    }
}

final class Recursive
{
    public function go()
    {
        return $this->walk(1, 2);
    }

    private function walk($a, $b)
    {
        return $a > 0 ? $this->walk($a - 1, $b) : 0;
    }
}

final class RecursiveUnused
{
    public function go()
    {
        return $this->walk(1, 2);
    }

    private function walk($a, $b)
    {
        return $a > 0 ? $this->walk($a - 1, 5) : 0;
    }
}
-----
<?php

class Base
{
    public function run($a, $b)
    {
    }
}

final class Child extends Base
{
    public function go()
    {
        $c = parent::run(...);
        return $this->run(1);
    }

    private function run($a)
    {
        return $a;
    }
}

final class Recursive
{
    public function go()
    {
        return $this->walk(1, 2);
    }

    private function walk($a, $b)
    {
        return $a > 0 ? $this->walk($a - 1, $b) : 0;
    }
}

final class RecursiveUnused
{
    public function go()
    {
        return $this->walk(1);
    }

    private function walk($a)
    {
        return $a > 0 ? $this->walk($a - 1) : 0;
    }
}
