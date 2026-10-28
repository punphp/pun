<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run([\stdClass::class, $name]);
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
        $this->run([\stdClass::class, $name]);
    }

    /**
     * @param string[] $items
     */
    private function run(array $items): void
    {
    }
}
