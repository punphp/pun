<?php

class SomeClass
{
    public function go(): void
    {
        $this->run($this->items());
    }

    /**
     * @return int[]
     */
    private function items(): iterable
    {
        return [];
    }

    private function run(array $items): void
    {
    }
}
-----
<?php

class SomeClass
{
    public function go(): void
    {
        $this->run($this->items());
    }

    /**
     * @return int[]
     */
    private function items(): iterable
    {
        return [];
    }

    /**
     * @param iterable<int> $items
     */
    private function run(array $items): void
    {
    }
}
