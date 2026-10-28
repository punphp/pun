<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run([new \ArrayIterator([1])]);
    }

    private function run(array $items): void
    {
    }
}
-----
<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run([new \ArrayIterator([1])]);
    }

    /**
     * @param \ArrayIterator<int, int>[] $items
     */
    private function run(array $items): void
    {
    }
}
