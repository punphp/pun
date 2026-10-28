<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $other->run(['a']);
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
        $other->run(['a']);
    }

    /**
     * @param string[] $items
     */
    private function run(array $items): void
    {
    }
}
