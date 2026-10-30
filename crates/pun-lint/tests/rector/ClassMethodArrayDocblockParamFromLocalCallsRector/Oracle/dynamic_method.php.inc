<?php

class SomeClass
{
    public function go(string $m): void
    {
        $this->run($this->{$m}());
    }

    private function run(array $items): void
    {
    }
}
