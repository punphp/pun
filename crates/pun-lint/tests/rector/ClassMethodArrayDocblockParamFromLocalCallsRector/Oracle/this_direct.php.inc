<?php

class SomeClass
{
    public function go(): void
    {
        $this->run($this);
    }

    private function run(array $items): void
    {
    }
}
