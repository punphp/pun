<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $items = ['a' => 1];
        if ($name) {
            $items['b'] = 2;
        }
        $this->run($items);
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
        $items = ['a' => 1];
        if ($name) {
            $items['b'] = 2;
        }
        $this->run($items);
    }

    /**
     * @param array<string, int> $items
     */
    private function run(array $items): void
    {
    }
}
