<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run(['a']);
    }

    /**
     * @param mixed[] $items
     */
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
        $this->run(['a']);
    }

    /**
     * @param string[] $items
     */
    private function run(array $items): void
    {
    }
}
