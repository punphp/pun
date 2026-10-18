<?php

final class A
{
    private $some;

    public function run()
    {
        return $this->getSome() + $this->GETSOME()->x + $this->other();
    }

    public function getSome()
    {
        return $this->some;
    }

    private function other()
    {
        return $this->a->b;
    }

    private function nop()
    {
        return $this->some;
        // trailing
    }

    public function run2()
    {
        return $this->nop();
    }
}
-----
<?php

final class A
{
    private $some;

    public function run()
    {
        return $this->some + $this->some->x + $this->a->b;
    }

    public function getSome()
    {
        return $this->some;
    }

    private function other()
    {
        return $this->a->b;
    }

    private function nop()
    {
        return $this->some;
        // trailing
    }

    public function run2()
    {
        return $this->nop();
    }
}
