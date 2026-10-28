<?php

class SomeClass
{
    public function go(string $s): void
    {
        $this->run([strlen($s)]);
    }

    private function run(array $items): void
    {
    }
}
-----
<?php

class SomeClass
{
    public function go(string $s): void
    {
        $this->run([strlen($s)]);
    }

    /**
     * @param int[] $items
     */
    private function run(array $items): void
    {
    }
}
