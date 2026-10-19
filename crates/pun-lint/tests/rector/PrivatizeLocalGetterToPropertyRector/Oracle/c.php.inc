<?php

$a = new class {
    private $name;

    public function run()
    {
        return $this->getName();
    }

    private function getName()
    {
        return $this->name;
    }
};

final class B
{
    private $name;

    public function run($other, $m)
    {
        $x = $other->getName();
        $y = $this->{$m}();
        $z = $this->withArg();
        $w = $this->isAbstract();
        $v = $this->notReturn();
        return $this->getName();
    }

    public function getName()
    {
        return $this->name;
    }

    private function withArg($a = 1)
    {
        return $this->name;
    }

    private function notReturn()
    {
        $this->name;
    }
}

final class C
{
    private $name;

    public function run()
    {
        return $this->isAbstract();
    }

    abstract private function isAbstract();
}
-----
<?php

$a = new class {
    private $name;

    public function run()
    {
        return $this->name;
    }

    private function getName()
    {
        return $this->name;
    }
};

final class B
{
    private $name;

    public function run($other, $m)
    {
        $x = $other->getName();
        $y = $this->{$m}();
        $z = $this->withArg();
        $w = $this->isAbstract();
        $v = $this->notReturn();
        return $this->name;
    }

    public function getName()
    {
        return $this->name;
    }

    private function withArg($a = 1)
    {
        return $this->name;
    }

    private function notReturn()
    {
        $this->name;
    }
}

final class C
{
    private $name;

    public function run()
    {
        return $this->isAbstract();
    }

    abstract private function isAbstract();
}
