<?php

class SomeClass
{
    /**
     * @param list<int> $x
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
     * @param list<int> $x
     */
    public function go(array $x): void
    {
        $this->run($x);
    }

    /**
     * @param int[] $items
     */
    private function run(array $items): void
    {
    }
}
