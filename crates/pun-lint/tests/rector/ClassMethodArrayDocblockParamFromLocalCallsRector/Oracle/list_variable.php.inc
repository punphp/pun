<?php

class SomeClass
{
    public function go(string $k): void
    {
        $x = [1, 2];
        $x[] = 3;
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
    public function go(string $k): void
    {
        $x = [1, 2];
        $x[] = 3;
        $this->run($x);
    }

    /**
     * @param int[] $items
     */
    private function run(array $items): void
    {
    }
}
