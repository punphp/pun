<?php

class SomeClass
{
    /**
     * @param array{a: array{b: int}} $x
     */
    public function go(array $x): void
    {
        $this->run($x);
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
     * @param array{a: array{b: int}} $x
     */
    public function go(array $x): void
    {
        $this->run($x);
    }

    /**
     * @param array<string, array<string, int>> $items
     */
    private function run(array $items): void
    {
    }
}
