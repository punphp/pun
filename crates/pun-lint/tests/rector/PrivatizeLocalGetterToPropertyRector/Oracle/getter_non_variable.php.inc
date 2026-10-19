<?php

final class A
{
    private $name;

    public function run()
    {
        return $this->other->getName() . (clone $this)->getName();
    }

    private function getName()
    {
        return $this->name;
    }
}
