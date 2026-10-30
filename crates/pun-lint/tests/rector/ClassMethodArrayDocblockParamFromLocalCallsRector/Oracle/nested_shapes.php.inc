<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run(['a' => ['b' => 1], 'c' => ['d' => 'x']]);
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
        $this->run(['a' => ['b' => 1], 'c' => ['d' => 'x']]);
    }

    /**
     * @param array<string, array<string, int>>|array<string, array<string, string>> $items
     */
    private function run(array $items): void
    {
    }
}
