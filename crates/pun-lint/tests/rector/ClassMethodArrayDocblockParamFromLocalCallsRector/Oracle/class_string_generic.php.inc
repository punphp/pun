<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run([get_class($other)]);
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
        $this->run([get_class($other)]);
    }

    /**
     * @param class-string<\SomeClass>[] $items
     */
    private function run(array $items): void
    {
    }
}
