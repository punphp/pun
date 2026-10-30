<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run(array_flip(['a']));
        $this->run(['k' => strlen($name)]);
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
        $this->run(array_flip(['a']));
        $this->run(['k' => strlen($name)]);
    }

    /**
     * @param array<string, int> $items
     */
    private function run(array $items): void
    {
    }
}
