<?php

class SomeClass
{
    public function go(array $ys): void
    {
        $x = [1];
        foreach ($ys as $y) {
            $x[] = 'a';
        }
        $this->run($x);
    }

    private function run(array $items): void
    {
    }
}
