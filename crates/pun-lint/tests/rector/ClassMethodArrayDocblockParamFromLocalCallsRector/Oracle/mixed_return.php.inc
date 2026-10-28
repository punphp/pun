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
    private function items(): mixed
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
    private function items(): mixed
    {
        return [];
    }

    /**
     * @param int[] $items
     */
    private function run(array $items): void
    {
    }
}
