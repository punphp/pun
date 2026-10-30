<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run([new UnknownClass()]);
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
        $this->run([new UnknownClass()]);
    }

    /**
     * @param \UnknownClass[] $items
     */
    private function run(array $items): void
    {
    }
}
