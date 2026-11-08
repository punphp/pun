<?php

abstract class SomeClass
{
    public function go(): void
    {
        $this->run(['a']);
    }

    abstract protected function run(array $items): void;
}
