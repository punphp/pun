<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run([is_int($name), (bool) $name]);
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
        $this->run([is_int($name), (bool) $name]);
    }

    /**
     * @param bool[] $items
     */
    private function run(array $items): void
    {
    }
}
