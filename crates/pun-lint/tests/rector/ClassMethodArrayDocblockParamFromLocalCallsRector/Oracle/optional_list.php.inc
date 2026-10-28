<?php

class SomeClass
{
    /**
     * @param list{0?: int, 1?: string} $x
     */
    public function go(array $x): void
    {
        $this->run($x);
    }

    private function run(array $items): void
    {
    }
}
