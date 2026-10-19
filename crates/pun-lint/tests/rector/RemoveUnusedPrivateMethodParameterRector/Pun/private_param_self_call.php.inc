<?php

final class SelfCall
{
    public function go()
    {
        return $this->f(1);
    }

    private function f($a)
    {
        return $this->f(2);
    }
}
