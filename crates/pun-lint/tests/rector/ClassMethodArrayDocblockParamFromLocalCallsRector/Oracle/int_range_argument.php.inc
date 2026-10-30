<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run([random_int(0, 10)]);
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
        $this->run([random_int(0, 10)]);
    }

    /**
     * @param int[] $items
     */
    private function run(array $items): void
    {
    }
}
