<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run(explode(',', $name));
        $this->run(array_keys(['a' => 1]));
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
        $this->run(explode(',', $name));
        $this->run(array_keys(['a' => 1]));
    }

    /**
     * @param string[] $items
     */
    private function run(array $items): void
    {
    }
}
