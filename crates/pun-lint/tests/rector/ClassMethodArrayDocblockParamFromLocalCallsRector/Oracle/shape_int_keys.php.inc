<?php

class SomeClass
{
    /**
     * @param array{1: int, 2: string} $x
     */
    public function go(array $x): void
    {
        $this->run($x);
    }

    private function run(array $items): void
    {
    }
}
-----
<?php

class SomeClass
{
    /**
     * @param array{1: int, 2: string} $x
     */
    public function go(array $x): void
    {
        $this->run($x);
    }

    /**
     * @param int[]|string[] $items
     */
    private function run(array $items): void
    {
    }
}
