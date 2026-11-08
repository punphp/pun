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
