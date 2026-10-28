<?php

class SomeClass
{
    public function go(string $k): void
    {
        $x = ['a' => 1];
        $x[$k] = 2;
        $this->run($x);
    }

    private function run(array $items): void
    {
    }
}
