<?php

class SomeClass
{
    /**
     * @param class-string $c
     */
    public function go(string $c): void
    {
        $this->run([$c]);
    }

    private function run(array $items): void
    {
    }
}
-----
<?php

class SomeClass
{
    /**
     * @param class-string $c
     */
    public function go(string $c): void
    {
        $this->run([$c]);
    }

    /**
     * @param class-string[] $items
     */
    private function run(array $items): void
    {
    }
}
