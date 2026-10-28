<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run([1.5 * strlen($name)]);
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
        $this->run([1.5 * strlen($name)]);
    }

    /**
     * @param float[] $items
     */
    private function run(array $items): void
    {
    }
}
