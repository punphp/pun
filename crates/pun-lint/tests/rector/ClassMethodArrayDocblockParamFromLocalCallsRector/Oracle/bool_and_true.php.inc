<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run([true]);
        $this->run([false]);
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
        $this->run([true]);
        $this->run([false]);
    }

    /**
     * @param bool[] $items
     */
    private function run(array $items): void
    {
    }
}
