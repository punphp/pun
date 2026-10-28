<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run([$this]);
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
        $this->run([$this]);
    }

    /**
     * @param $this[] $items
     */
    private function run(array $items): void
    {
    }
}
