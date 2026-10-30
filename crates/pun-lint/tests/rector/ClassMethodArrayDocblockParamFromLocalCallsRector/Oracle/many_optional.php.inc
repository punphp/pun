<?php

class SomeClass
{
    /**
     * @param array{a?: int, b?: int, c?: int, d?: int, e?: int, f?: int, g?: int, h?: int, i?: int, j?: int, k?: string} $x
     */
    public function go(array $x): void
    {
        $this->run($x);
    }

    private function run(array $items): void
    {
    }
}
